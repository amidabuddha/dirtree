# dirtree

A simple Rust command-line tool to display directory structures in a tree-like format, similar to the Unix `tree` command.

## Clean Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed (includes Cargo), with support for the Rust 2024 edition declared in `Cargo.toml`
- Git to clone the repository

No environment variables or secrets are required.

### Build from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/amidabuddha/dirtree
   cd dirtree
   ```

2. Restore dependencies and build the release binary using the existing lockfile:
   ```bash
   cargo build --locked --release
   ```

   Cargo automatically downloads any required crates and builds them. This project currently has no external dependencies, so there is no separate dependency-installation step. Keep `Cargo.lock`; `--locked` prevents Cargo from changing it.

3. Run the local binary:
   ```bash
   ./target/release/dirtree --help
   ./target/release/dirtree .
   ```

These commands build and run the checkout without a global installation or publishing step. Binary paths assume Cargo's default project-local `target/` directory; if you override the target directory, use the corresponding binary path.

## Normal Development

Run these commands from the repository root:

```bash
cargo build --locked
cargo run --locked -- -a src
cargo test --locked
```

`cargo build` incrementally builds the debug binary in `target/debug/`. `cargo run` rebuilds as needed before running; arguments after `--` go to dirtree. `cargo test` runs the unit tests in `src/main.rs` and the CLI integration tests in `tests/cli.rs`. No watch command is configured in this repository; rerun the relevant command after editing.

Ordinary source changes do not require deleting dependencies or build output. To incrementally rebuild the optimized binary, use `cargo build --locked --release`.

## Clean Rebuild of an Existing Checkout

The repository ignores `/target`, Cargo's generated build directory, and contains no custom build-output configuration. Before cleaning, confirm that `target/` contains only this checkout's generated output and is not a symlink to shared files. From the repository root, run:

```bash
cargo clean --target-dir target
cargo build --locked --release --target-dir target
./target/release/dirtree --help
```

The explicit `--target-dir target` scopes both cleanup and rebuilding to this checkout's `target/`, even if your environment or Cargo configuration overrides the default output directory. Cleaning removes its debug and release build artifacts; the next build regenerates the release output and automatically restores any missing required crates using `Cargo.lock`. If `target/` has already been removed, skip the clean command and build directly.

Preserve `Cargo.lock`, source, configuration, any `.env` files and user data. Do not clear global Cargo caches, remove rustup or installed targets, or delete installed executables. A clean rebuild does not require reinstalling the tool globally.

## Usage

```bash
./target/release/dirtree [OPTIONS] [PATH]
```

If dirtree is already installed on your `PATH`, you can use `dirtree` instead of the local binary path.

### Options

- `-a`: Show hidden files and directories
- `-U`: Leave entries unsorted
- `--help`: Display help message and exit

### Example

From a Rust project directory:

```console
$ dirtree
.
├── Cargo.toml
├── README.md
└── src
    └── main.rs
```

## Contributing

Feel free to submit issues or pull requests to improve the tool!

## License

This project is licensed under the MIT License - see the LICENSE file for details.
