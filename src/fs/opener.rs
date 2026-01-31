use crate::config::opener::OpenerConfig;
use anyhow::Result;
use std::path::Path;
use std::process::Command;

/// ファイルを適切なアプリケーションで開く
pub fn open_file(path: &Path, config: &OpenerConfig) -> Result<()> {
    if !path.exists() {
        return Err(anyhow::anyhow!("File not found: {}", path.display()));
    }

    // ディレクトリの場合はエラー
    if path.is_dir() {
        return Err(anyhow::anyhow!("Cannot open directory: {}", path.display()));
    }

    // 拡張子ベースのマッチング
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if let Some(opener) = config.extension.get(ext) {
            return execute_opener(opener, path);
        }
    }

    // デフォルトのオープナーを使用
    execute_opener(&config.default, path)
}

/// エディタでファイルを開く
pub fn open_with_editor(path: &Path, config: &OpenerConfig) -> Result<()> {
    if !path.exists() {
        return Err(anyhow::anyhow!("File not found: {}", path.display()));
    }

    execute_opener(&config.editor, path)
}

/// ページャでファイルを開く
pub fn open_with_pager(path: &Path, config: &OpenerConfig) -> Result<()> {
    if !path.exists() {
        return Err(anyhow::anyhow!("File not found: {}", path.display()));
    }

    execute_opener(&config.pager, path)
}

/// オープナーコマンドを実行
fn execute_opener(opener: &str, path: &Path) -> Result<()> {
    // コマンドとオプションを分割
    let parts: Vec<&str> = opener.split_whitespace().collect();
    if parts.is_empty() {
        return Err(anyhow::anyhow!("Empty opener command"));
    }

    let cmd = parts[0];
    let args = &parts[1..];

    // コマンドを実行（バックグラウンド）
    let mut command = Command::new(cmd);
    command.args(args);
    command.arg(path);

    // 標準出力・エラー出力を無視
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        use std::process::Stdio;

        command.stdin(Stdio::null());
        command.stdout(Stdio::null());
        command.stderr(Stdio::null());

        // デタッチして実行
        unsafe {
            command.pre_exec(|| {
                // 新しいプロセスグループを作成
                libc::setsid();
                Ok(())
            });
        }
    }

    // Windows環境でも出力を抑制
    #[cfg(not(unix))]
    {
        use std::process::Stdio;
        command.stdin(Stdio::null());
        command.stdout(Stdio::null());
        command.stderr(Stdio::null());
    }

    // コマンドを起動し、エラーメッセージを改善
    command.spawn().map_err(|e| {
        anyhow::anyhow!("Failed to execute '{}': {}", cmd, e)
    })?;

    Ok(())
}

/// エディタでファイルを編集（フォアグラウンド実行、TUI一時停止が必要）
pub fn edit_file_foreground(path: &Path, editor: &str) -> Result<()> {
    if !path.exists() {
        return Err(anyhow::anyhow!("File not found: {}", path.display()));
    }

    // コマンドとオプションを分割
    let parts: Vec<&str> = editor.split_whitespace().collect();
    if parts.is_empty() {
        return Err(anyhow::anyhow!("Empty editor command"));
    }

    let cmd = parts[0];
    let args = &parts[1..];

    // コマンドをフォアグラウンドで実行
    let status = Command::new(cmd)
        .args(args)
        .arg(path)
        .status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("Editor exited with error: {}", status));
    }

    Ok(())
}

/// ファイルをシェルで実行（実行可能ファイル用）
pub fn execute_file(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(anyhow::anyhow!("File not found: {}", path.display()));
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = std::fs::metadata(path)?;
        let permissions = metadata.permissions();

        // 実行可能かチェック
        if permissions.mode() & 0o111 == 0 {
            return Err(anyhow::anyhow!("File is not executable: {}", path.display()));
        }
    }

    // 実行可能ファイルを起動
    Command::new(path).spawn()?;

    Ok(())
}
