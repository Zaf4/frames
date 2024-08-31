from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING, Literal

from polars import scan_ipc, read_ipc

from framex._dicts import _REMOTE_DATASETS

# from framex._dicts._constants import _EXTENSION, _LOCAL_DIR

if TYPE_CHECKING:
    import os

    from polars import LazyFrame


def _save(
    frame: LazyFrame,
    path: Path[str],
    format: str | Literal["feather", "parquet", "csv", "json", "ipc"] = "feather",
) -> None:
    """
    Saves a LazyFrame to the desired format.

    Parameters
    ----------
    frame : polars.LazyFrame
        The frame to save.
    name : str
        The name of the dataset.
    path : Path[str]
        The path to save the dataset to.
    format : str or Literal["feather", "parquet", "csv", "json", "ipc"], optional
        The format to save the dataset in.
        Default is "feather"
    """
    if format in ("feather", "ipc"):
        frame.write_ipc(path, compression="zstd")
    elif format == "parquet":
        frame.write_parquet(path, compression="zstd")
    elif format == "csv":
        frame.write_csv(path)
    elif format == "json":
        frame.write_ndjson(path)


def get(
    name: str,
    *,
    dir: str | Path[str] | os.pathlike[str] | None = None,
    overwrite: bool = False,
    format: str | Literal["feather", "parquet", "csv", "json", "ipc"] = "feather",
) -> None:
    """
    Loads dataset by with the given name if available.

    Parameters
    ----------
    name : str
        Name of the dataset to load.

    dir : str, optional
        Directory to save the dataset to.

    overwrite : bool, optional
        Whether to overwrite the dataset if it already exists.
        Default is True

    format : str or Literal["feather", "parquet", "csv", "json", "ipc"], optional
        The format to save the dataset in.
        Default is "feather"

    Returns
    -------
    None
    """
    if dir is None:
        Path().resolve()
    else:
        dir = Path(dir).resolve()
    # select the function to load the dataset
    loader = read_ipc
    # check if the dataset is available
    if name not in _REMOTE_DATASETS:
        msg = f"Dataset {name} not found."
        raise ValueError(msg)

    else:
        frame = loader(_REMOTE_DATASETS.get(name))

    # list of all files in the directory
    dir = Path(dir).resolve()
    cached = list(dir.glob(f"*{format}"))
    path = dir / f"{name}.{format}"

    if path in cached:
        if not overwrite:
            msg = f"File {path} already exists.\nUse `overwrite` to overwrite."
            print(msg)
            return
        else:
            print(f"Overwriting: {path}\n")
            _save(frame=frame, path=path, format=format)
    elif path not in cached:
        _save(frame=frame, path=path, format=format)
        print(f"Saving: {path}\n")

    return


if __name__ == "__main__":
    pass
