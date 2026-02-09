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
        crate::config::get_echofiler_config_dir().join("bookmarks.toml")
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
