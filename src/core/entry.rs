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

        // 隠しファイル判定
        let is_hidden = Self::check_hidden(path, &name);
        let is_symlink = metadata.is_symlink();

        #[cfg(unix)]
        let is_executable = {
            let mode = metadata.permissions().mode();
            mode & 0o111 != 0
        };

        #[cfg(windows)]
        let is_executable = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| {
                matches!(
                    e.to_lowercase().as_str(),
                    "exe" | "bat" | "cmd" | "com" | "ps1"
                )
            })
            .unwrap_or(false);

        #[cfg(not(any(unix, windows)))]
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

    /// 隠しファイル判定（OS別）
    fn check_hidden(path: &Path, name: &str) -> bool {
        // ドットファイルは全OSで隠しファイルとして扱う
        if name.starts_with('.') {
            return true;
        }

        // Windows: FILE_ATTRIBUTE_HIDDEN属性をチェック
        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt;
            const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;
            if let Ok(metadata) = fs::metadata(path) {
                return metadata.file_attributes() & FILE_ATTRIBUTE_HIDDEN != 0;
            }
        }

        #[cfg(not(windows))]
        let _ = path;

        false
    }

    /// DirEntryから直接構築（read_dir最適化用、余計なsyscallを回避）
    fn from_dir_entry(dir_entry: &fs::DirEntry) -> std::io::Result<Self> {
        let path = dir_entry.path();
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        // DirEntry::metadata()はOSによってはsymlink_metadataより高速
        // （Linuxではd_typeが使える場合がある）
        let file_type = dir_entry.file_type()?;
        let is_dir = file_type.is_dir();
        let is_symlink = file_type.is_symlink();

        // メタデータは必要な場合のみ取得
        let metadata = dir_entry.metadata()?;

        let is_hidden = Self::check_hidden(&path, &name);

        #[cfg(unix)]
        let is_executable = {
            let mode = metadata.permissions().mode();
            mode & 0o111 != 0
        };

        #[cfg(windows)]
        let is_executable = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| {
                matches!(
                    e.to_lowercase().as_str(),
                    "exe" | "bat" | "cmd" | "com" | "ps1"
                )
            })
            .unwrap_or(false);

        #[cfg(not(any(unix, windows)))]
        let is_executable = false;

        Ok(Self {
            path,
            name,
            is_dir,
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
            if let Ok(file_entry) = Self::from_dir_entry(&entry) {
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
