#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Visual,
    Command,
    Search,
    Bookmark,
    BookmarkPrefix,  // gキー（プレフィックスキー）を押したときに入る
    BookmarkSelect,  // g m でブックマーク一覧表示 + キーで移動
    Help,
    Confirm,
    MessageDialog,
}

impl Default for InputMode {
    fn default() -> Self {
        Self::Normal
    }
}
