# FrameX CLI (Rust)

A Rust rewrite of the FrameX command-line interface. It is a drop-in
replacement for the Python `fx` command and talks to the same
[datavil/datasets](https://github.com/datavil/datasets) source and the same
local cache (`~/.cache/framex/datasets`).

The Python CLI (`framex/cli/`) is kept as-is; this crate is purely additive.

## Build

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
