use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NEXT_TEMP_DIR: AtomicU64 = AtomicU64::new(0);

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new() -> Self {
        let base = std::env::temp_dir();

        for _ in 0..100 {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos();
            let sequence = NEXT_TEMP_DIR.fetch_add(1, Ordering::Relaxed);
            let path = base.join(format!(
                "dirtree-cli-test-{}-{timestamp}-{sequence}",
                process::id()
            ));

            match fs::create_dir(&path) {
                Ok(()) => return Self { path },
                Err(error) if error.kind() == ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("failed to create temporary directory: {error}"),
            }
        }

        panic!("failed to create a unique temporary directory");
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
fn help_succeeds_on_stdout() {
    let output = Command::new(env!("CARGO_BIN_EXE_dirtree"))
        .arg("--help")
        .output()
        .expect("failed to run dirtree");

    assert!(output.status.success());
    assert!(
        std::str::from_utf8(&output.stdout)
            .expect("stdout should be UTF-8")
            .contains("Usage: dirtree")
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn nonexistent_path_exits_with_error() {
    let temp_dir = TempDir::new();
    let missing_path = temp_dir.path().join("missing");
    let output = Command::new(env!("CARGO_BIN_EXE_dirtree"))
        .arg(&missing_path)
        .output()
        .expect("failed to run dirtree");

    assert_eq!(output.status.code(), Some(1));
    assert!(
        std::str::from_utf8(&output.stderr)
            .expect("stderr should be UTF-8")
            .contains("does not exist")
    );
}

#[test]
fn regular_file_exits_with_error() {
    let temp_dir = TempDir::new();
    let file_path = temp_dir.path().join("file.txt");
    fs::write(&file_path, b"not a directory").expect("failed to create regular file");

    let output = Command::new(env!("CARGO_BIN_EXE_dirtree"))
        .arg(&file_path)
        .output()
        .expect("failed to run dirtree");

    assert_eq!(output.status.code(), Some(1));
    assert!(
        std::str::from_utf8(&output.stderr)
            .expect("stderr should be UTF-8")
            .contains("is not a directory")
    );
}

#[test]
fn invalid_option_exits_with_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_dirtree"))
        .arg("--this-option-does-not-exist")
        .output()
        .expect("failed to run dirtree");

    assert_eq!(output.status.code(), Some(1));
    assert!(
        std::str::from_utf8(&output.stderr)
            .expect("stderr should be UTF-8")
            .contains("Unknown option")
    );
}
