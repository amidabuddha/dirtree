# dirtree

A simple Rust command-line tool to display directory structures in a tree-like format, similar to the Unix `tree` command.

## Features
- Displays directory contents in a hierarchical tree view
- Optional `-a` flag to show hidden files and directories (those starting with `.`)
- Optional `-U` flag to disable alphabetical sorting
- Defaults to current directory if no path is provided
- Cross-platform compatibility

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

### Examples

List current directory (no hidden files, sorted):
```bash
dirtree
```

Output:
```
.
├── Cargo.toml
└── src
    └── main.rs
```

List a specific directory with hidden files and no sorting:
```bash
dirtree -a -U ./my-rust-app
```

Output:
```
/path/to/dir/my-rust-app
├── Cargo.toml
├── .gitignore
├── .git
│   └── (git contents...)
└── src
    └── main.rs
```

## Contributing

Feel free to submit issues or pull requests to improve the tool!

## License

This project is licensed under the MIT License - see the LICENSE file for details.
