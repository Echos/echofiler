use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use super::PaneSide;

/// 終了時の状態
///
/// 次回起動時に同じディレクトリから開始するために保存する。
/// 設定ファイルではなく実行時の状態なのでデータディレクトリに置く
/// (`${XDG_DATA_HOME:-~/.local/share}/echofiler/session.toml`)。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Session {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left_dir: Option<PathBuf>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right_dir: Option<PathBuf>,
    /// アクティブだったペイン ("left" / "right")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_pane: Option<String>,
}

impl Session {
    pub fn load() -> Result<Self> {
        let path = Self::get_session_path();
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::get_session_path();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&path, toml::to_string_pretty(self)?)?;
        Ok(())
    }

    /// 現在の状態からセッションを組み立てる
    pub fn from_dirs(left_dir: PathBuf, right_dir: PathBuf, active_pane: PaneSide) -> Self {
        Self {
            left_dir: Some(left_dir),
            right_dir: Some(right_dir),
            active_pane: Some(
                match active_pane {
                    PaneSide::Left => "left",
                    PaneSide::Right => "right",
                }
                .to_string(),
            ),
        }
    }

    /// 復元に使えるディレクトリを返す
    ///
    /// 保存後に削除・リネームされている場合があるため、
    /// 現存するディレクトリだけを返す（呼び出し側でカレントディレクトリにフォールバックする）。
    pub fn dir(&self, side: PaneSide) -> Option<PathBuf> {
        let dir = match side {
            PaneSide::Left => self.left_dir.as_ref(),
            PaneSide::Right => self.right_dir.as_ref(),
        }?;

        if dir.is_dir() {
            Some(dir.clone())
        } else {
            None
        }
    }

    /// 復元するアクティブペイン
    pub fn active_side(&self) -> Option<PaneSide> {
        match self.active_pane.as_deref() {
            Some("left") => Some(PaneSide::Left),
            Some("right") => Some(PaneSide::Right),
            _ => None,
        }
    }

    fn get_session_path() -> PathBuf {
        if let Ok(data_home) = std::env::var("XDG_DATA_HOME") {
            PathBuf::from(data_home).join("echofiler/session.toml")
        } else if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".local/share/echofiler/session.toml")
        } else {
            PathBuf::from("session.toml")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_dirs_roundtrips_through_toml() {
        let session = Session::from_dirs(
            PathBuf::from("/tmp"),
            PathBuf::from("/usr"),
            PaneSide::Right,
        );

        let text = toml::to_string_pretty(&session).unwrap();
        let restored: Session = toml::from_str(&text).unwrap();

        assert_eq!(restored.left_dir, Some(PathBuf::from("/tmp")));
        assert_eq!(restored.right_dir, Some(PathBuf::from("/usr")));
        assert_eq!(restored.active_side(), Some(PaneSide::Right));
    }

    #[test]
    fn missing_directories_are_not_restored() {
        let session = Session::from_dirs(
            PathBuf::from("/tmp"),
            PathBuf::from("/no/such/directory-for-echofiler-test"),
            PaneSide::Left,
        );

        assert_eq!(session.dir(PaneSide::Left), Some(PathBuf::from("/tmp")));
        // 存在しないディレクトリは復元対象にしない
        assert_eq!(session.dir(PaneSide::Right), None);
    }

    #[test]
    fn empty_session_restores_nothing() {
        let session = Session::default();
        assert_eq!(session.dir(PaneSide::Left), None);
        assert_eq!(session.dir(PaneSide::Right), None);
        assert_eq!(session.active_side(), None);
    }

    #[test]
    fn unknown_active_pane_is_ignored() {
        let session: Session = toml::from_str("active_pane = \"middle\"").unwrap();
        assert_eq!(session.active_side(), None);
    }
}
