use anyhow::{Context, Result};
use std::fs::{self, File};
use std::path::{Path, PathBuf};

#[cfg(feature = "archive")]
use flate2::read::GzDecoder;
#[cfg(feature = "archive")]
use tar::Archive as TarArchive;
#[cfg(feature = "archive")]
use zip::ZipArchive;

/// アーカイブファイルかどうかを拡張子から判定
pub fn is_archive(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        matches!(
            ext.to_lowercase().as_str(),
            "zip" | "tar" | "gz" | "tgz" | "bz2" | "xz" | "7z"
        )
    } else {
        // .tar.gz のような複合拡張子をチェック
        if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
            let lower = filename.to_lowercase();
            lower.ends_with(".tar.gz")
                || lower.ends_with(".tar.bz2")
                || lower.ends_with(".tar.xz")
        } else {
            false
        }
    }
}

/// アーカイブファイルの内容を一覧表示
#[cfg(feature = "archive")]
pub fn list_archive_contents(path: &Path) -> Result<Vec<String>> {
    let filename = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    if filename.ends_with(".zip") {
        list_zip_contents(path)
    } else if filename.ends_with(".tar") {
        list_tar_contents(path)
    } else if filename.ends_with(".tar.gz") || filename.ends_with(".tgz") {
        list_tar_gz_contents(path)
    } else {
        Ok(vec!["Unsupported archive format".to_string()])
    }
}

#[cfg(not(feature = "archive"))]
pub fn list_archive_contents(_path: &Path) -> Result<Vec<String>> {
    Ok(vec![
        "Archive support requires 'archive' feature".to_string()
    ])
}

/// ZIPファイルの内容を一覧表示
#[cfg(feature = "archive")]
fn list_zip_contents(path: &Path) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;

    let mut contents = Vec::new();
    for i in 0..archive.len() {
        if let Ok(file) = archive.by_index(i) {
            let name = file.name().to_string();
            let size = file.size();
            contents.push(format!("{} ({} bytes)", name, size));
        }
    }

    Ok(contents)
}

/// TARファイルの内容を一覧表示
#[cfg(feature = "archive")]
fn list_tar_contents(path: &Path) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let mut archive = TarArchive::new(file);

    let mut contents = Vec::new();
    for entry in archive.entries()? {
        if let Ok(entry) = entry {
            let path = entry.path()?;
            let size = entry.size();
            contents.push(format!("{} ({} bytes)", path.display(), size));
        }
    }

    Ok(contents)
}

/// TAR.GZファイルの内容を一覧表示
#[cfg(feature = "archive")]
fn list_tar_gz_contents(path: &Path) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = TarArchive::new(decoder);

    let mut contents = Vec::new();
    for entry in archive.entries()? {
        if let Ok(entry) = entry {
            let path = entry.path()?;
            let size = entry.size();
            contents.push(format!("{} ({} bytes)", path.display(), size));
        }
    }

    Ok(contents)
}

/// アーカイブを展開
#[cfg(feature = "archive")]
pub fn extract_archive(archive_path: &Path, dest_dir: &Path) -> Result<()> {
    let filename = archive_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    // 展開先ディレクトリを作成
    fs::create_dir_all(dest_dir)?;

    if filename.ends_with(".zip") {
        extract_zip(archive_path, dest_dir)
    } else if filename.ends_with(".tar") {
        extract_tar(archive_path, dest_dir)
    } else if filename.ends_with(".tar.gz") || filename.ends_with(".tgz") {
        extract_tar_gz(archive_path, dest_dir)
    } else {
        Err(anyhow::anyhow!("Unsupported archive format"))
    }
}

#[cfg(not(feature = "archive"))]
pub fn extract_archive(_archive_path: &Path, _dest_dir: &Path) -> Result<()> {
    Err(anyhow::anyhow!(
        "Archive support requires 'archive' feature"
    ))
}

/// ZIPファイルを展開
#[cfg(feature = "archive")]
fn extract_zip(archive_path: &Path, dest_dir: &Path) -> Result<()> {
    let file = File::open(archive_path)?;
    let mut archive = ZipArchive::new(file)?;

    archive.extract(dest_dir)?;

    Ok(())
}

/// TARファイルを展開
#[cfg(feature = "archive")]
fn extract_tar(archive_path: &Path, dest_dir: &Path) -> Result<()> {
    let file = File::open(archive_path)?;
    let mut archive = TarArchive::new(file);

    archive.unpack(dest_dir)?;

    Ok(())
}

/// TAR.GZファイルを展開
#[cfg(feature = "archive")]
fn extract_tar_gz(archive_path: &Path, dest_dir: &Path) -> Result<()> {
    let file = File::open(archive_path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = TarArchive::new(decoder);

    archive.unpack(dest_dir)?;

    Ok(())
}

/// ファイルをZIPに圧縮
#[cfg(feature = "archive")]
pub fn compress_to_zip(files: &[PathBuf], output_path: &Path) -> Result<()> {
    let file = File::create(output_path)?;
    let mut zip = zip::ZipWriter::new(file);

    let options = zip::write::FileOptions::<()>::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    for file_path in files {
        if file_path.is_file() {
            let name = file_path
                .file_name()
                .and_then(|s| s.to_str())
                .context("Invalid filename")?;

            zip.start_file(name, options)?;
            let content = fs::read(file_path)?;
            std::io::copy(&mut content.as_slice(), &mut zip)?;
        }
    }

    zip.finish()?;

    Ok(())
}

#[cfg(not(feature = "archive"))]
pub fn compress_to_zip(_files: &[PathBuf], _output_path: &Path) -> Result<()> {
    Err(anyhow::anyhow!(
        "Archive support requires 'archive' feature"
    ))
}
