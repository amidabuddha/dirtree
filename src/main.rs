use std::env;
use std::fs;
use std::path::Path;

// Function to print directory structure with indentation
fn print_dir_structure(path: &Path, prefix: String, show_hidden: bool) {
    if let Ok(entries) = fs::read_dir(path) {
        let mut entries: Vec<_> = entries
            .filter_map(|e| e.ok())
            .filter(|e| {
                let name = e.file_name();
                let name_str = name.to_string_lossy();
                show_hidden || !name_str.starts_with('.')
            })
            .collect();

        // Sort entries alphabetically
        entries.sort_by(|a, b| a.path().cmp(&b.path()));

        for (i, entry) in entries.iter().enumerate() {
            let is_last = i == entries.len() - 1;
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();

            // Print current entry
            println!(
                "{}{} {}",
                prefix,
                if is_last { "└──" } else { "├──" },
                name
            );

            // If it's a directory, recurse with updated prefix
            if entry.file_type().unwrap().is_dir() {
                let new_prefix = format!(
                    "{}{}    ",
                    prefix,
                    if is_last { " " } else { "│" }
                );
                print_dir_structure(&entry.path(), new_prefix, show_hidden);
            }
        }
    }
}

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Parse arguments
    let mut show_hidden = false;
    let mut dir_path = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" => {
                show_hidden = true;
                i += 1;
            }
            path => {
                if dir_path.is_none() {
                    dir_path = Some(path);
                    i += 1;
                } else {
                    println!("Error: Multiple directory paths specified");
                    std::process::exit(1);
                }
            }
        }
    }

    // Use current directory if no path is provided
    let dir_path = match dir_path {
        Some(path) => path.to_string(),
        None => ".".to_string(),
    };

    let path = Path::new(&dir_path);

    // Verify path exists and is a directory
    if !path.exists() {
        println!("Error: Path '{}' does not exist", dir_path);
        std::process::exit(1);
    }

    if !path.is_dir() {
        println!("Error: '{}' is not a directory", dir_path);
        std::process::exit(1);
    }

    // Print root directory: "." if no path provided, otherwise the path
    if dir_path == "." {
        println!(".");
    } else {
        println!("{}", path.canonicalize().unwrap().file_name().unwrap().to_string_lossy());
    }

    // Start printing directory structure
    print_dir_structure(path, String::new(), show_hidden);
}