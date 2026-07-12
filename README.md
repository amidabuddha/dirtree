# dirtree

A simple Rust command-line tool to display directory structures in a tree-like format, similar to the Unix `tree` command.

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) installed (includes Cargo)

### Build from Source
1. Clone the repository:
   ```bash
   git clone https://github.com/amidabuddha/dirtree
   cd dirtree
   ```

2. Build the release binary:
   ```bash
   cargo build --release
   ```

3. (Optional) Install globally:
   ```bash
   sudo cp target/release/dirtree /usr/local/bin/
   ```

## Usage

```bash
dirtree [OPTIONS] [PATH]
```

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
