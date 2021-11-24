# moni

![LOC](https://img.shields.io/tokei/lines/github.com/UltiRequiem/moni?color=blue&label=Total%20Lines)

Asynchronously and recursively delete all the apparitions of a directory/file.

## Usage

```sh
moni
```

This will delete all the `["node_modules", "__pycache__", "dist", "build"]` and some more,
check the full list [here](https://github.com/UltiRequiem/moni/blob/main/src/main.rs#L6),
in your current folder and subfolders.

Built in help:

```sh
$ moni --help
moni v0.1.0

UltiRequiem <https://github.com/UltiRequiem>

Delete directories to free up disk space.

USAGE:
    moni

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

## Installation

With `cargo`:

```sh
cargo install moni
```

Or use a binary from [releases](https://github.com/UltiRequiem/moni/releases/latest).

## License

Licensed under the [MIT License](./license).
