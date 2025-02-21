use std::env;
use std::fs;
use std::path::Path;

fn print_dir_structure(path: &Path, prefix: String, sort: bool, show_hidden: bool) {
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
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();

            println!(
                "{}{} {}",
                prefix,
                if is_last { "└──" } else { "├──" },
                name
            );

            if entry.file_type().unwrap().is_dir() {
                let new_prefix = format!(
                    "{}{}    ",
                    prefix,
                    if is_last { " " } else { "│" }
                );
                print_dir_structure(&entry.path(), new_prefix, sort, show_hidden);
            }
        }
    }
}

fn print_help() {
    println!("Usage: dirtree [OPTIONS] [PATH]");
    println!();
    println!("Options:");
    println!("  -a         Show hidden files");
    println!("  -U         Leave files unsorted.");
    println!("  --help     Display this help");
    println!();
    println!("Default PATH is '.' (current directory).");
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
                print_help();
                std::process::exit(0);
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

    // Print full canonical path, or "." if no path provided
    let root_name = if dir_path == "." {
        ".".to_string()
    } else {
        path.canonicalize()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| dir_path.to_string())
    };
    println!("{}", root_name);

    print_dir_structure(path, String::new(), sort, show_hidden);
}