use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;

#[derive(Debug, PartialEq, Eq)]
struct Config {
    show_hidden: bool,
    sort: bool,
    dir_path: PathBuf,
}

#[derive(Debug, PartialEq, Eq)]
enum ParsedArgs {
    Help,
    Run(Config),
}

fn parse_args<I>(args: I) -> Result<ParsedArgs, String>
where
    I: IntoIterator<Item = OsString>,
{
    let mut show_hidden = false;
    let mut sort = true;
    let mut dir_path = None;
    let mut options_finished = false;

    for arg in args {
        if !options_finished {
            if arg.as_os_str() == OsStr::new("--help") {
                return Ok(ParsedArgs::Help);
            }
            if arg.as_os_str() == OsStr::new("--") {
                options_finished = true;
                continue;
            }
            if arg.as_os_str() == OsStr::new("-a") {
                show_hidden = true;
                continue;
            }
            if arg.as_os_str() == OsStr::new("-U") {
                sort = false;
                continue;
            }
            if starts_with_dash(arg.as_os_str()) {
                return Err(format!("Unknown option: {}", arg.to_string_lossy()));
            }
        }

        if dir_path.replace(PathBuf::from(arg)).is_some() {
            return Err("Multiple paths specified".to_string());
        }
    }

    Ok(ParsedArgs::Run(Config {
        show_hidden,
        sort,
        dir_path: dir_path.unwrap_or_else(|| PathBuf::from(".")),
    }))
}

fn print_dir_structure<W: Write>(
    path: &Path,
    prefix: &mut String,
    sort: bool,
    show_hidden: bool,
    writer: &mut W,
) -> io::Result<()> {
    let read_dir = fs::read_dir(path).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("Failed to read '{}': {error}", path.display()),
        )
    })?;
    let mut entries = Vec::new();

    for entry in read_dir {
        let entry = entry.map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("Failed to read an entry in '{}': {error}", path.display()),
            )
        })?;
        let name = entry.file_name();
        if show_hidden || !is_hidden(&name) {
            entries.push((name, entry));
        }
    }

    if sort {
        entries.sort_unstable_by(|(left_name, _), (right_name, _)| left_name.cmp(right_name));
    }

    for (i, (name, entry)) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let (connector, child_prefix) = if is_last {
            ("└──", "    ")
        } else {
            ("├──", "│   ")
        };

        writeln!(writer, "{prefix}{connector} {}", name.to_string_lossy())?;

        let entry_path = entry.path();
        let file_type = entry.file_type().map_err(|error| {
            io::Error::new(
                error.kind(),
                format!(
                    "Failed to read metadata for '{}': {error}",
                    entry_path.display()
                ),
            )
        })?;
        if file_type.is_dir() {
            let old_len = prefix.len();
            prefix.push_str(child_prefix);

            print_dir_structure(&entry_path, prefix, sort, show_hidden, writer)?;

            prefix.truncate(old_len);
        }
    }

    Ok(())
}

fn is_hidden(name: &OsStr) -> bool {
    name.as_encoded_bytes().starts_with(b".")
}

fn starts_with_dash(arg: &OsStr) -> bool {
    arg.as_encoded_bytes().starts_with(b"-")
}

fn print_help<W: Write>(writer: &mut W) -> io::Result<()> {
    writeln!(
        writer,
        r#"Usage: dirtree [OPTIONS] [PATH]

Options:
  -a        Show hidden files
  -U        Leave entries unsorted
  --help    Display this help

Default PATH is '.' (current directory)."#
    )
}

fn print_tree<W: Write>(config: &Config, writer: &mut W) -> io::Result<()> {
    let path = &config.dir_path;
    let metadata = fs::metadata(path).map_err(|error| {
        let message = if error.kind() == io::ErrorKind::NotFound {
            format!("Path '{}' does not exist", path.display())
        } else {
            format!("Failed to inspect '{}': {error}", path.display())
        };
        io::Error::new(error.kind(), message)
    })?;

    if !metadata.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("'{}' is not a directory", path.display()),
        ));
    }

    let root_name = if path == Path::new(".") {
        ".".to_string()
    } else {
        path.canonicalize()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|_| path.display().to_string())
    };
    writeln!(writer, "{root_name}")?;

    let mut prefix = String::new();
    print_dir_structure(path, &mut prefix, config.sort, config.show_hidden, writer)
}

fn run<I, W>(args: I, writer: &mut W) -> io::Result<()>
where
    I: IntoIterator<Item = OsString>,
    W: Write,
{
    let parsed =
        parse_args(args).map_err(|message| io::Error::new(io::ErrorKind::InvalidInput, message))?;

    match parsed {
        ParsedArgs::Help => print_help(writer)?,
        ParsedArgs::Run(config) => print_tree(&config, writer)?,
    }

    Ok(())
}

fn main() {
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());

    if let Err(error) = run(env::args_os().skip(1), &mut writer) {
        if error.kind() == io::ErrorKind::BrokenPipe {
            return;
        }

        let _ = writer.flush();
        eprintln!("Error: {error}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        fn new() -> Self {
            let base = env::temp_dir();

            for attempt in 0..100 {
                let nanos = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("system clock should be after the Unix epoch")
                    .as_nanos();
                let path = base.join(format!("dirtree-test-{}-{nanos}-{attempt}", process::id()));

                if fs::create_dir(&path).is_ok() {
                    return Self { path };
                }
            }

            panic!("failed to create temporary test directory");
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn parse_defaults_to_current_directory() {
        let parsed = parse_args(Vec::new()).expect("parse should succeed");

        assert_eq!(
            parsed,
            ParsedArgs::Run(Config {
                show_hidden: false,
                sort: true,
                dir_path: PathBuf::from("."),
            })
        );
    }

    #[test]
    fn parse_rejects_unknown_options() {
        let error = parse_args([OsString::from("--bad-option")]).expect_err("parse should fail");

        assert_eq!(error, "Unknown option: --bad-option");
    }

    #[test]
    fn parse_accepts_dash_prefixed_path_after_separator() {
        let parsed = parse_args([OsString::from("--"), OsString::from("-fixtures")])
            .expect("parse should succeed");

        assert_eq!(
            parsed,
            ParsedArgs::Run(Config {
                show_hidden: false,
                sort: true,
                dir_path: PathBuf::from("-fixtures"),
            })
        );
    }

    #[test]
    fn parse_rejects_multiple_paths() {
        let error = parse_args([OsString::from("first"), OsString::from("second")])
            .expect_err("parse should fail");

        assert_eq!(error, "Multiple paths specified");
    }

    #[cfg(unix)]
    #[test]
    fn parse_accepts_non_utf8_paths() {
        use std::os::unix::ffi::OsStringExt;

        let raw_path = OsString::from_vec(vec![b'p', b'a', b't', b'h', 0xff]);
        let parsed = parse_args([raw_path.clone()]).expect("parse should succeed");

        match parsed {
            ParsedArgs::Run(config) => assert_eq!(config.dir_path.as_os_str(), raw_path),
            ParsedArgs::Help => panic!("expected runnable config"),
        }
    }

    #[test]
    fn print_tree_filters_hidden_entries_by_default() {
        let temp_dir = TempDir::new();
        fs::write(temp_dir.path().join(".hidden"), "").expect("write hidden file");
        fs::write(temp_dir.path().join("visible.txt"), "").expect("write visible file");

        let config = Config {
            show_hidden: false,
            sort: true,
            dir_path: temp_dir.path().to_path_buf(),
        };
        let mut output = Vec::new();

        print_tree(&config, &mut output).expect("tree should print");

        let output = String::from_utf8(output).expect("tree output should be UTF-8");
        assert!(output.contains("visible.txt"));
        assert!(!output.contains(".hidden"));
    }

    #[test]
    fn print_dir_structure_returns_read_dir_errors() {
        let temp_dir = TempDir::new();
        let file_path = temp_dir.path().join("file.txt");
        fs::write(&file_path, "").expect("write file");

        let mut prefix = String::new();
        let mut output = Vec::new();
        let error = print_dir_structure(&file_path, &mut prefix, true, true, &mut output)
            .expect_err("read_dir error should be returned");

        assert!(error.to_string().contains("Failed to read"));
    }
}
