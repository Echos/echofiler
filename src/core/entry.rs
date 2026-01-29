use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[derive(Debug, Clone)]
pub struct Entry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub is_hidden: bool,
    pub is_symlink: bool,
    pub is_executable: bool,
    pub size: u64,
    pub modified: Option<SystemTime>,
}

impl Entry {
    pub fn from_path(path: &Path) -> std::io::Result<Self> {
        let metadata = fs::symlink_metadata(path)?;
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        let is_hidden = name.starts_with('.');
        let is_symlink = metadata.is_symlink();

        #[cfg(unix)]
        let is_executable = {
            let mode = metadata.permissions().mode();
            mode & 0o111 != 0
        };

        #[cfg(not(unix))]
        let is_executable = false;

        Ok(Self {
            path: path.to_path_buf(),
            name,
            is_dir: metadata.is_dir(),
            is_hidden,
            is_symlink,
            is_executable,
            size: metadata.len(),
            modified: metadata.modified().ok(),
        })
    }

    pub fn read_dir(path: &Path) -> std::io::Result<Vec<Self>> {
        let mut entries = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if let Ok(file_entry) = Self::from_path(&entry.path()) {
                entries.push(file_entry);
            }
        }

        entries.sort_by(|a, b| {
            match (a.is_dir, b.is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            }
        });

        Ok(entries)
    }
}
