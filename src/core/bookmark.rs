use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub name: String,
    pub path: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<char>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BookmarkList {
    pub bookmarks: Vec<Bookmark>,
}

/// ブックマーク登録の入力を「名前」と「ジャンプキー」に分解する
///
/// `"Home h"` -> `("Home", Some('h'))` / `"Home"` -> `("Home", None)`
/// 末尾が「スペース + 1文字」の場合のみキー指定とみなす。
pub fn parse_bookmark_input(input: &str) -> Option<(String, Option<char>)> {
    let input = input.trim();
    if input.is_empty() {
        return None;
    }

    if let Some((name, key)) = input.rsplit_once(' ') {
        let name = name.trim();
        let mut chars = key.chars();
        // 「最後の1文字」の判定は文字数で行う（len()はバイト数のため日本語1文字で誤判定する）
        if let (Some(key_char), None) = (chars.next(), chars.next()) {
            if !name.is_empty() {
                return Some((name.to_string(), Some(key_char)));
            }
        }
    }

    Some((input.to_string(), None))
}

impl BookmarkList {
    pub fn load() -> Result<Self> {
        let path = Self::get_bookmark_path();

        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let list: BookmarkList = toml::from_str(&content)?;
            Ok(list)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::get_bookmark_path();

        // ディレクトリが存在しない場合は作成
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }

    pub fn add(&mut self, name: String, path: PathBuf) {
        // 同じパスが既に存在する場合は更新
        if let Some(bookmark) = self.bookmarks.iter_mut().find(|b| b.path == path) {
            bookmark.name = name;
        } else {
            self.bookmarks.push(Bookmark { name, path, key: None });
        }
    }

    pub fn add_with_key(&mut self, name: String, path: PathBuf, key: char) {
        // 同じキーが既に存在する場合は上書き
        if let Some(bookmark) = self.bookmarks.iter_mut().find(|b| b.key == Some(key)) {
            bookmark.name = name;
            bookmark.path = path;
        } else {
            self.bookmarks.push(Bookmark { name, path, key: Some(key) });
        }
    }

    pub fn find_by_key(&self, key: char) -> Option<&Bookmark> {
        self.bookmarks.iter().find(|b| b.key == Some(key))
    }

    pub fn remove(&mut self, index: usize) -> Option<Bookmark> {
        if index < self.bookmarks.len() {
            Some(self.bookmarks.remove(index))
        } else {
            None
        }
    }

    fn get_bookmark_path() -> PathBuf {
        if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(config_home).join("echofiler/bookmarks.toml")
        } else if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".config/echofiler/bookmarks.toml")
        } else {
            PathBuf::from("bookmarks.toml")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bookmark_creation() {
        let bookmark = Bookmark {
            name: "Test".to_string(),
            path: PathBuf::from("/tmp/test"),
            key: None,
        };

        assert_eq!(bookmark.name, "Test");
        assert_eq!(bookmark.path, PathBuf::from("/tmp/test"));
        assert_eq!(bookmark.key, None);
    }

    #[test]
    fn test_bookmark_list_default() {
        let list = BookmarkList::default();
        assert_eq!(list.bookmarks.len(), 0);
    }

    #[test]
    fn test_bookmark_add() {
        let mut list = BookmarkList::default();

        list.add("Home".to_string(), PathBuf::from("/home/user"));
        assert_eq!(list.bookmarks.len(), 1);
        assert_eq!(list.bookmarks[0].name, "Home");
        assert_eq!(list.bookmarks[0].path, PathBuf::from("/home/user"));
    }

    #[test]
    fn test_bookmark_add_duplicate_path() {
        let mut list = BookmarkList::default();

        // 同じパスを追加
        list.add("First".to_string(), PathBuf::from("/tmp"));
        list.add("Second".to_string(), PathBuf::from("/tmp"));

        // 同じパスの場合は上書きされる
        assert_eq!(list.bookmarks.len(), 1);
        assert_eq!(list.bookmarks[0].name, "Second");
    }

    #[test]
    fn test_parse_bookmark_input() {
        // 名前 + キー
        assert_eq!(
            parse_bookmark_input("Home h"),
            Some(("Home".to_string(), Some('h')))
        );
        // 大文字のキーもそのまま扱う
        assert_eq!(
            parse_bookmark_input("Home H"),
            Some(("Home".to_string(), Some('H')))
        );
        // スペースを含む名前
        assert_eq!(
            parse_bookmark_input("My Documents d"),
            Some(("My Documents".to_string(), Some('d')))
        );
        // 名前のみ（キーなし）
        assert_eq!(parse_bookmark_input("Projects"), Some(("Projects".to_string(), None)));
        // 1文字だけの入力は「名前のみ」。キーとして登録するには "名前 キー" が必要
        assert_eq!(parse_bookmark_input("H"), Some(("H".to_string(), None)));
        // 空入力
        assert_eq!(parse_bookmark_input("   "), None);
        // 日本語1文字のキー（len()ではなく文字数で判定する）
        assert_eq!(
            parse_bookmark_input("ホーム あ"),
            Some(("ホーム".to_string(), Some('あ')))
        );
    }

    #[test]
    fn test_find_by_key_is_case_sensitive() {
        let mut list = BookmarkList::default();
        list.add_with_key("Home".to_string(), PathBuf::from("/home/user"), 'H');

        assert_eq!(list.find_by_key('H').map(|b| b.name.as_str()), Some("Home"));
        // 大文字と小文字は別のキーとして扱う
        assert!(list.find_by_key('h').is_none());
    }

    #[test]
    fn test_bookmark_remove() {
        let mut list = BookmarkList::default();

        list.add("First".to_string(), PathBuf::from("/tmp/1"));
        list.add("Second".to_string(), PathBuf::from("/tmp/2"));
        assert_eq!(list.bookmarks.len(), 2);

        // 削除
        let removed = list.remove(0);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().name, "First");
        assert_eq!(list.bookmarks.len(), 1);
        assert_eq!(list.bookmarks[0].name, "Second");
    }

    #[test]
    fn test_bookmark_remove_invalid_index() {
        let mut list = BookmarkList::default();
        list.add("Test".to_string(), PathBuf::from("/tmp"));

        // 範囲外のインデックス
        let removed = list.remove(10);
        assert!(removed.is_none());
        assert_eq!(list.bookmarks.len(), 1);
    }

    #[test]
    fn test_bookmark_serialization() {
        let mut list = BookmarkList::default();
        list.add("Home".to_string(), PathBuf::from("/home/user"));
        list.add("Documents".to_string(), PathBuf::from("/home/user/Documents"));

        // TOML形式にシリアライズ
        let toml_str = toml::to_string(&list).unwrap();
        assert!(toml_str.contains("Home"));
        assert!(toml_str.contains("Documents"));

        // デシリアライズ
        let deserialized: BookmarkList = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized.bookmarks.len(), 2);
        assert_eq!(deserialized.bookmarks[0].name, "Home");
        assert_eq!(deserialized.bookmarks[1].name, "Documents");
    }
}
