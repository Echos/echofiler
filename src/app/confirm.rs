use std::path::PathBuf;

/// 確認待ちの操作
#[derive(Debug, Clone)]
pub enum PendingAction {
    /// ファイル削除
    Delete { paths: Vec<PathBuf> },
    /// ファイル貼り付け（上書きの可能性）
    Paste { has_conflicts: bool },
    /// アーカイブ展開
    ExtractArchive { archive_path: PathBuf, dest_dir: PathBuf },
    /// 別のペインにコピー
    CopyToOtherPane { paths: Vec<PathBuf>, dest_dir: PathBuf },
    /// 別のペインに移動
    MoveToOtherPane { paths: Vec<PathBuf>, dest_dir: PathBuf },
    /// アプリケーション終了
    Quit,
}

impl PendingAction {
    /// 確認メッセージを取得
    pub fn message(&self) -> String {
        match self {
            PendingAction::Delete { paths } => {
                if paths.len() == 1 {
                    format!("Delete '{}'?", paths[0].display())
                } else {
                    format!("Delete {} items?", paths.len())
                }
            }
            PendingAction::Paste { has_conflicts } => {
                if *has_conflicts {
                    "Overwrite existing files?".to_string()
                } else {
                    "Paste files?".to_string()
                }
            }
            PendingAction::ExtractArchive { archive_path, .. } => {
                format!("Extract '{}'?", archive_path.display())
            }
            PendingAction::CopyToOtherPane { paths, .. } => {
                if paths.len() == 1 {
                    format!("Copy '{}' to other pane?", paths[0].display())
                } else {
                    format!("Copy {} items to other pane?", paths.len())
                }
            }
            PendingAction::MoveToOtherPane { paths, .. } => {
                if paths.len() == 1 {
                    format!("Move '{}' to other pane?", paths[0].display())
                } else {
                    format!("Move {} items to other pane?", paths.len())
                }
            }
            PendingAction::Quit => {
                "Quit echofiler?".to_string()
            }
        }
    }
}
