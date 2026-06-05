"""
Behavioral parity between the Python `fx` CLI and the Rust rewrite (cli-rs/).

Runs the same commands through both binaries and compares behavior:
- **exact** (color-stripped) stdout where the two must agree (version, about,
  list, errors, get/bring messages with the temp dir normalized);
- **structural** properties where rendering legitimately differs (produced-file
  shape and columns, `show`/`describe` table shape).

Run standalone::

    poetry run python tests/test_parity.py

Requires the Python `fx` on PATH and the built Rust binary
(`cli-rs/target/release/fx`). Skips gracefully if either is missing or the
dataset catalog/network is unavailable.
"""

from __future__ import annotations

import re
import shutil
import subprocess
import tempfile
from pathlib import Path

import polars as pl

_REPO = Path(__file__).resolve().parent.parent
_PY = shutil.which("fx")
_RS = next(
    (
        path
        for path in (
            _REPO / "cli-rs" / "target" / "release" / "fx",
            _REPO / "cli-rs" / "target" / "debug" / "fx",
        )
        if path.exists()
    ),
    None,
)

_ANSI = re.compile(r"\x1b\[[0-9;]*m")
_SHAPE = re.compile(r"shape: \(\d+, \d+\)")


def green(text: str) -> str:  # noqa: D103
    return f"\033[32m{text}\033[0m"


def red(text: str) -> str:  # noqa: D103
    return f"\033[31m{text}\033[0m"


def _strip(text: str) -> str:
    """Remove ANSI escapes so output can be compared as plain text."""
    return _ANSI.sub("", text)


def _run(binary: str | Path, args: list[str], cwd: str | None = None) -> str:
    """Run a CLI and return its combined, color-stripped output."""
    result = subprocess.run(
        [str(binary), *args],
        capture_output=True,
        text=True,
        cwd=cwd,
    )
    return _strip(result.stdout + result.stderr)


def _norm(text: str, directory: Path) -> str:
    """Replace a temp directory (and its `/private` macOS form) with `<DIR>`."""
    return text.replace(f"/private{directory}", "<DIR>").replace(str(directory), "<DIR>")


def _shape(output: str) -> str | None:
    """Extract the polars `shape: (r, c)` line, if present."""
    match = _SHAPE.search(output)
    return match.group(0) if match else None


def _exact(checks: list[tuple[str, bool]], label: str, args: list[str]) -> None:
    """Assert both CLIs emit byte-identical (color-stripped) output."""
    checks.append((label, _run(_PY, args) == _run(_RS, args)))


def _check_get_sequence(checks: list[tuple[str, bool]]) -> None:
    """Saved -> already-exists -> Overwritten messages, with dirs normalized."""
    with tempfile.TemporaryDirectory() as py_dir, tempfile.TemporaryDirectory() as rs_dir:

        def sequence(binary: str | Path, directory: str) -> str:
            args = ["get", "titanic", "--dir", directory, "--format", "csv"]
            first = _run(binary, args)
            second = _run(binary, args)
            third = _run(binary, [*args, "--overwrite"])
            return first + second + third

        py_out = _norm(sequence(_PY, py_dir), Path(py_dir))
        rs_out = _norm(sequence(_RS, rs_dir), Path(rs_dir))
        checks.append(("get saved/exists/overwritten", py_out == rs_out))


def _check_produced_file(checks: list[tuple[str, bool]]) -> None:
    """The CSV both CLIs write must have the same shape and columns."""
    with tempfile.TemporaryDirectory() as py_dir, tempfile.TemporaryDirectory() as rs_dir:
        _run(_PY, ["get", "titanic", "--dir", py_dir, "--format", "csv"])
        _run(_RS, ["get", "titanic", "--dir", rs_dir, "--format", "csv"])
        py_frame = pl.read_csv(Path(py_dir) / "titanic.csv")
        rs_frame = pl.read_csv(Path(rs_dir) / "titanic.csv")
        same = py_frame.shape == rs_frame.shape and py_frame.columns == rs_frame.columns
        checks.append(("get produces identical csv shape/columns", same))


def _check_missing_dir(checks: list[tuple[str, bool]]) -> None:
    """`get` into a missing directory errors identically (path normalized)."""
    missing = Path(tempfile.gettempdir()) / "framex_parity_missing_xyz"
    args = ["get", "iris", "--dir", str(missing)]
    py_out = _norm(_run(_PY, args), missing)
    rs_out = _norm(_run(_RS, args), missing)
    checks.append(("get missing-dir error", py_out == rs_out))


def _check_bring_not_found(checks: list[tuple[str, bool]]) -> None:
    """`bring` of an unknown dataset errors identically."""
    with tempfile.TemporaryDirectory() as directory:
        args = ["bring", "framex_no_such_dataset", "--dir", directory]
        py_out = _norm(_run(_PY, args), Path(directory))
        rs_out = _norm(_run(_RS, args), Path(directory))
        checks.append(("bring not-found error", py_out == rs_out))


def _check_shape(checks: list[tuple[str, bool]], label: str, args: list[str]) -> None:
    """`show`/`describe` render the same `shape: (r, c)` (rendering may differ)."""
    py_shape = _shape(_run(_PY, args))
    rs_shape = _shape(_run(_RS, args))
    checks.append((label, py_shape is not None and py_shape == rs_shape))


def parity_checks() -> list[tuple[str, bool]]:
    """Run every parity check and return a list of (label, passed)."""
    checks: list[tuple[str, bool]] = []

    # Offline: works without the catalog.
    _exact(checks, "fx --version", ["--version"])

    # Catalog/network availability (the cache may serve this offline).
    if _run(_PY, ["list", "--remote"]).strip() == "":
        print("SKIP: catalog/network unavailable, skipping network checks")
        return checks

    # Exact (color-stripped) stdout.
    _exact(checks, "about titanic", ["about", "titanic"])
    _exact(checks, "about mpg", ["about", "mpg"])
    _exact(checks, "about unknown (error)", ["about", "framex_no_such_dataset"])
    _exact(checks, "list --remote", ["list", "--remote"])
    _exact(checks, "list --local", ["list", "--local"])
    _exact(checks, "list iris (includes)", ["list", "iris"])
    _exact(checks, "show unknown (error)", ["show", "framex_no_such_dataset"])

    # Messages with paths (normalized) and produced-file data.
    _check_get_sequence(checks)
    _check_missing_dir(checks)
    _check_bring_not_found(checks)
    _check_produced_file(checks)

    # Structural (table shape) — rendering / dtypes legitimately differ.
    _check_shape(checks, "show iris shape", ["show", "iris"])
    _check_shape(checks, "describe titanic shape", ["describe", "titanic"])

    return checks


def main() -> None:
    """Run the parity suite and print a report."""
    if _PY is None or _RS is None:
        missing = "Python `fx`" if _PY is None else "Rust `cli-rs/target/release/fx`"
        print(red(f"SKIP: {missing} not found (build with `cargo build --release` in cli-rs/)"))
        return

    checks = parity_checks()
    for label, passed in checks:
        mark = green("PASS") if passed else red("FAIL")
        print(f"{mark}  {label}")

    failures = [label for label, passed in checks if not passed]
    print()
    print(f"{len(checks) - len(failures)}/{len(checks)} parity checks passed")
    assert not failures, f"parity mismatch: {failures}"


if __name__ == "__main__":
    main()
