# dirtree

A simple Rust command-line tool to display directory structures in a tree-like format, similar to the Unix `tree` command.

## Features
- Displays directory contents in a hierarchical tree view
- Optional `-h` flag to show hidden files and directories (those starting with `.`)
- Optional `-s` flag to sort directory contents alphabetically
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

- `-h`: Show hidden files and directories
- `-s`: Sort entries alphabetically
- `-H, --help`: Display help message and exit

### Examples

List current directory (no hidden files):
```bash
dirtree
```

Output:
```
my-rust-app
├── Cargo.toml
└── src
    └── main.rs
```

List current directory with hidden files:
```bash
dirtree -h
```

Output:
```
my-rust-app
├── .git
│   └── (git contents...)
├── .gitignore
├── Cargo.toml
└── src
    └── main.rs
```

List a specific directory:
```bash
dirtree -h -s ./my-rust-app
```

## Contributing

Feel free to submit issues or pull requests to improve the tool!

## License

This project is licensed under the MIT License - see the LICENSE file for details.
