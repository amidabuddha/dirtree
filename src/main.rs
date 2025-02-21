use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn print_dir_structure<W: Write>(
    path: &Path,
    prefix: &mut String,
    sort: bool,
    show_hidden: bool,
    writer: &mut W,
) {
    if let Ok(entries) = fs::read_dir(path) {
        let mut entries: Vec<_> = entries
            .filter_map(|e| {
                let e = e.ok()?;
                let name = e.file_name();
                let name_str = name.to_string_lossy();
                if show_hidden || !name_str.starts_with('.') {
                    Some(e)
                } else {
                    None
                }
            })
            .collect();

        if sort {
            entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
        }

        for (i, entry) in entries.iter().enumerate() {
            let is_last = i == entries.len() - 1;
            let name = entry.file_name().to_string_lossy().into_owned(); // Convert to owned String

            // Write the current entry line
            writeln!(writer, "{}{} {}", prefix, if is_last { "└──" } else { "├──" }, name)
                .expect("Failed to write entry");

            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    // Append to the prefix for the next level
                    let old_len = prefix.len();
                    if is_last {
                        prefix.push_str("    "); // Append spaces for last entry
                    } else {
                        prefix.push_str("│   "); // Append vertical bar and spaces
                    }

                    // Recurse into the directory
                    print_dir_structure(&entry.path(), prefix, sort, show_hidden, writer);

                    // Truncate the prefix back to its original length
                    prefix.truncate(old_len);
                }
            }
        }
    }
}

fn print_help<W: Write>(writer: &mut W) {
    writeln!(writer, "Usage: dirtree [OPTIONS] [PATH]").unwrap();
    writeln!(writer).unwrap();
    writeln!(writer, "Options:").unwrap();
    writeln!(writer, "  -a        Show hidden files").unwrap();
    writeln!(writer, "  -U        Sort alphabetically").unwrap();
    writeln!(writer, "  --help    Display this help").unwrap();
    writeln!(writer).unwrap();
    writeln!(writer, "Default PATH is '.' (current directory).").unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut show_hidden = false;
    let mut sort = true;
    let mut dir_path = None;

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-a" => show_hidden = true,
            "-U" => sort = false,
            "--help" => {
                let stdout = io::stdout();
                let mut writer = io::BufWriter::new(stdout.lock());
                print_help(&mut writer);
                return;
            }
            path => {
                if dir_path.replace(path).is_some() {
                    println!("Error: Multiple paths specified");
                    std::process::exit(1);
                }
            }
        }
    }

    let dir_path = dir_path.unwrap_or(".");
    let path = Path::new(dir_path);

    if !path.exists() {
        println!("Error: Path '{}' does not exist", dir_path);
        std::process::exit(1);
    }
    if !path.is_dir() {
        println!("Error: '{}' is not a directory", dir_path);
        std::process::exit(1);
    }

    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());

    let root_name = if dir_path == "." {
        ".".to_string()
    } else {
        path.canonicalize()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| dir_path.to_string())
    };
    writeln!(writer, "{}", root_name).unwrap();

    let mut prefix = String::new();
    print_dir_structure(path, &mut prefix, sort, show_hidden, &mut writer);
}