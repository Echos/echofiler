use crate::config::opener::OpenerConfig;
use anyhow::Result;
use std::path::Path;
use std::process::Command;

/// 端末を必要とするオープナー（フォアグラウンドで起動しないと動作しない）
const TERMINAL_OPENERS: &[&str] = &[
    "vi", "vim", "nvim", "nano", "emacs", "micro", "helix", "hx", "kak", "ne",
    "less", "more", "most", "bat", "man",
];

/// パスに対して使われるオープナーコマンドを解決する
pub fn resolve_opener<'a>(path: &Path, config: &'a OpenerConfig) -> &'a str {
    path.extension()
        .and_then(|e| e.to_str())
        .and_then(|ext| config.extension.get(ext))
        .map(|s| s.as_str())
        .unwrap_or(&config.default)
}

/// オープナーが端末を必要とするコマンドかどうかを判定する
///
/// これらはデタッチして起動すると端末を失って動作しないため、
/// TUIを一時停止してフォアグラウンドで実行する必要がある。
pub fn is_terminal_opener(opener: &str) -> bool {
    opener
        .split_whitespace()
        .next()
        .map(|cmd| {
            // パス付きで指定された場合もコマンド名で判定する
            let name = Path::new(cmd)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(cmd);
            TERMINAL_OPENERS.contains(&name)
        })
        .unwrap_or(false)
}

/// ファイルを適切なアプリケーションで開く
pub fn open_file(path: &Path, config: &OpenerConfig) -> Result<()> {
    if !path.exists() {
        return Err(anyhow::anyhow!("File not found: {}", path.display()));
    }

    // ディレクトリの場合はエラー
    if path.is_dir() {
        return Err(anyhow::anyhow!("Cannot open directory: {}", path.display()));
    }

    execute_opener(resolve_opener(path, config), path)
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

    command.spawn().map_err(|e| opener_error(cmd, e))?;

    Ok(())
}

/// オープナー起動失敗時のエラーを組み立てる
///
/// spawn/status の ENOENT は「開くファイルが無い」ではなく
/// 「オープナーコマンドが見つからない」を意味するため、区別して伝える。
fn opener_error(cmd: &str, e: std::io::Error) -> anyhow::Error {
    if e.kind() == std::io::ErrorKind::NotFound {
        anyhow::anyhow!(
            "Opener command not found: '{}' (check opener.toml)",
            cmd
        )
    } else {
        anyhow::anyhow!("Failed to run opener '{}': {}", cmd, e)
    }
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
        .status()
        .map_err(|e| opener_error(cmd, e))?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn config() -> OpenerConfig {
        let mut config = OpenerConfig::default();
        config.default = "xdg-open".to_string();
        config.extension.insert("toml".to_string(), "vi".to_string());
        config
            .extension
            .insert("png".to_string(), "xdg-open".to_string());
        config
    }

    #[test]
    fn resolve_opener_prefers_extension_over_default() {
        let config = config();
        assert_eq!(
            resolve_opener(&PathBuf::from("/tmp/a.toml"), &config),
            "vi"
        );
        assert_eq!(
            resolve_opener(&PathBuf::from("/tmp/a.png"), &config),
            "xdg-open"
        );
        // 未登録の拡張子・拡張子なしは既定のオープナー
        assert_eq!(
            resolve_opener(&PathBuf::from("/tmp/a.unknown"), &config),
            "xdg-open"
        );
        assert_eq!(resolve_opener(&PathBuf::from("/tmp/README"), &config), "xdg-open");
    }

    #[test]
    fn terminal_openers_are_detected() {
        // 端末を必要とするものはフォアグラウンド起動が必要
        assert!(is_terminal_opener("vi"));
        assert!(is_terminal_opener("nvim"));
        assert!(is_terminal_opener("less"));
        assert!(is_terminal_opener("/usr/bin/vim"));
        assert!(is_terminal_opener("less -R"));

        // GUI・デタッチして良いもの
        assert!(!is_terminal_opener("xdg-open"));
        assert!(!is_terminal_opener("wslview"));
        assert!(!is_terminal_opener("feh"));
        assert!(!is_terminal_opener(""));
    }

    #[test]
    fn missing_opener_command_is_reported_as_such() {
        // ENOENT は「ファイルが無い」ではなく「オープナーが無い」と伝える
        let err = opener_error(
            "no-such-opener",
            std::io::Error::from(std::io::ErrorKind::NotFound),
        );
        let message = err.to_string();
        assert!(
            message.contains("Opener command not found") && message.contains("no-such-opener"),
            "想定外のメッセージ: {message}"
        );
        assert!(!message.contains("No such file or directory"), "{message}");
    }
}
