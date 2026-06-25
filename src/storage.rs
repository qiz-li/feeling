use crate::model::Entry;
use fs2::FileExt;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

const HEADER: &str = "date,feeling";
const CHECKSUM_PREFIX: &str = "#sha256:";

pub fn default_data_path() -> PathBuf {
    let data_dir = std::env::var("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            PathBuf::from(home).join(".local/share")
        });

    data_dir.join("feeling").join("feeling.csv")
}

pub fn read_entries(path: &Path) -> Result<Vec<Entry>, StorageError> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(path).map_err(StorageError::Io)?;
    let lines: Vec<&str> = content.lines().collect();

    if lines.is_empty() {
        return Ok(Vec::new());
    }

    // Verify checksum if present
    if let Some(last) = lines.last()
        && let Some(stored_hash) = last.strip_prefix(CHECKSUM_PREFIX)
    {
        let data_content: String = lines[..lines.len() - 1].join("\n") + "\n";
        let computed = compute_checksum(data_content.as_bytes());
        if computed != stored_hash {
            return Err(StorageError::ChecksumMismatch);
        }
    }

    let mut entries = Vec::new();
    for line in &lines {
        if *line == HEADER || line.starts_with(CHECKSUM_PREFIX) {
            continue;
        }
        match Entry::from_str(line) {
            Ok(entry) => entries.push(entry),
            Err(_) => eprintln!("warning: skipping invalid line: {line}"),
        }
    }

    Ok(entries)
}

pub fn write_entries(path: &Path, entries: &[Entry]) -> Result<(), StorageError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(StorageError::Io)?;
    }

    let lock_path = path.with_extension("lock");
    let lock_file = File::create(&lock_path).map_err(StorageError::Io)?;
    lock_file.lock_exclusive().map_err(StorageError::Io)?;
    // Ensures the lock is released and the lock file removed on every exit path,
    // including the early returns from the `?` operators below.
    let _lock = LockGuard {
        file: &lock_file,
        path: &lock_path,
    };

    rotate_backups(path)?;

    let mut content = String::from(HEADER);
    content.push('\n');
    for entry in entries {
        content.push_str(&entry.to_csv_row());
        content.push('\n');
    }

    let checksum = compute_checksum(content.as_bytes());
    content.push_str(CHECKSUM_PREFIX);
    content.push_str(&checksum);
    content.push('\n');

    atomic_write(path, content.as_bytes())?;

    Ok(())
}

struct LockGuard<'a> {
    file: &'a File,
    path: &'a Path,
}

impl Drop for LockGuard<'_> {
    fn drop(&mut self) {
        let _ = FileExt::unlock(self.file);
        let _ = fs::remove_file(self.path);
    }
}

fn atomic_write(path: &Path, data: &[u8]) -> Result<(), StorageError> {
    let tmp = path.with_extension(format!("tmp.{}", std::process::id()));
    let mut file = File::create(&tmp).map_err(StorageError::Io)?;
    file.write_all(data).map_err(StorageError::Io)?;
    file.sync_all().map_err(StorageError::Io)?;
    fs::rename(&tmp, path).map_err(StorageError::Io)?;
    Ok(())
}

fn rotate_backups(path: &Path) -> Result<(), StorageError> {
    if !path.exists() {
        return Ok(());
    }
    let p = path.to_path_buf();
    let bak3 = p.with_extension("csv.3");
    let bak2 = p.with_extension("csv.2");
    let bak1 = p.with_extension("csv.1");

    let _ = fs::remove_file(&bak3);
    if bak2.exists() {
        let _ = fs::rename(&bak2, &bak3);
    }
    if bak1.exists() {
        let _ = fs::rename(&bak1, &bak2);
    }
    let _ = fs::copy(path, &bak1);

    Ok(())
}

fn compute_checksum(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

pub fn migrate_legacy(new_path: &Path) {
    if new_path.exists() {
        return;
    }
    let legacy = dirs_legacy();
    if legacy.exists() {
        if let Some(parent) = new_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::copy(&legacy, new_path);
        eprintln!(
            "migrated data from {} to {}",
            legacy.display(),
            new_path.display()
        );
    }
}

fn dirs_legacy() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".config/feeling/feelings.csv")
}

#[derive(Debug)]
pub enum StorageError {
    Io(std::io::Error),
    ChecksumMismatch,
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "storage I/O error: {e}"),
            Self::ChecksumMismatch => write!(f, "data file checksum mismatch — file may be corrupted"),
        }
    }
}

impl std::error::Error for StorageError {}
