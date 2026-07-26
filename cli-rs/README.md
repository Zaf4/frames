# FrameX CLI (Rust)

A Rust rewrite of the FrameX command-line interface. It is a drop-in
replacement for the Python `fx` command and talks to the same
[datavil/datasets](https://github.com/datavil/datasets) source and the same
local cache (`~/.cache/framex/datasets`).

The Python CLI (`framex/cli/`) is kept as-is; this crate is purely additive.

## Install

### Install script (Linux and macOS)

```shell
curl -LsSf https://framex.datavil.org/install.sh | sh
```

The script verifies the release archive's SHA-256 checksum and installs `fx`
to `~/.local/bin`. Set `FX_INSTALL_DIR` to choose another directory, or
`FX_VERSION` to install a specific release:

```shell
curl -LsSf https://framex.datavil.org/install.sh \
  | FX_VERSION=1.0.3 FX_INSTALL_DIR="$HOME/bin" sh
```

On Windows PowerShell:

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://framex.datavil.org/install.ps1 | iex"
```

### Prebuilt binaries

Each `v*` tag triggers `.github/workflows/build-cli.yml`, which builds and
attaches binaries to the GitHub Release for:

| Platform | Target | Asset |
| --- | --- | --- |
| Linux (x86-64) | `x86_64-unknown-linux-gnu` | `fx-…-linux-gnu.tar.gz` |
| Linux (ARM64) | `aarch64-unknown-linux-gnu` | `fx-…-linux-gnu.tar.gz` |
| Alpine Linux (x86-64) | `x86_64-unknown-linux-musl` | `fx-…-linux-musl.tar.gz` |
| Alpine Linux (ARM64) | `aarch64-unknown-linux-musl` | `fx-…-linux-musl.tar.gz` |
| macOS (Apple Silicon) | `aarch64-apple-darwin` | `fx-…-apple-darwin.tar.gz` |
| macOS (Intel) | `x86_64-apple-darwin` | `fx-…-apple-darwin.tar.gz` |
| Windows (x86-64) | `x86_64-pc-windows-msvc` | `fx-…-windows-msvc.zip` |
| Windows (ARM64) | `aarch64-pc-windows-msvc` | `fx-…-windows-msvc.zip` |

Each archive ships a single `fx` (`fx.exe` on Windows) plus a `.sha256`.
Download, extract, and put it on your `PATH`. The workflow can also be run
manually from the Actions tab (`workflow_dispatch`).

### Build from source

```shell
cargo build --release
```

The binary lands at `target/release/fx`. Put it on your `PATH` (or
`cargo install --path .`) to use it as `fx`.

> First build pulls in `polars` and takes a few minutes; later builds are fast.

## Commands

Same surface as the Python CLI:

| Command | Description |
| --- | --- |
| `fx get <datasets...>` | Download dataset(s) and save them in a chosen `--format`. |
| `fx bring <datasets...>` | Copy dataset(s) from the local cache to a directory. |
| `fx about <datasets...>` | Print dataset metadata from the remote info CSV. |
| `fx list [includes]` | List datasets (`--all` / `--remote` / `--local`). |
| `fx show <dataset>` | Print a preview of a dataset. |
| `fx describe <dataset>` | Print summary statistics for a dataset. |
| `fx --version` | Show version. |

`get` / `bring` flags: `--dir/-d`, `--format/-f` (default `csv`),
`--overwrite/-o`; `get` also takes `--cache/-c`.

## Layout

The module layout mirrors the Python package:

```
src/
  main.rs            clap CLI + dispatch        (← _entry.py)
  colors.rs          ANSI helpers               (← utils/_colors.py)
  errors.rs          error enum                 (← utils/_exceptions.py)
  constants.rs       cache dir, extension, URLs (← _dicts/_constants.py)
  catalog.rs         remote/local discovery     (← _dicts/*)
  load.rs            load() + save()            (← datasets/core.py, _save)
  commands/          one module per subcommand
```
