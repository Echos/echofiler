use super::Entry;
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortMethod {
    Name,
    Size,
    Modified,
    Extension,
}

impl SortMethod {
    pub fn next(&self) -> Self {
        match self {
            SortMethod::Name => SortMethod::Size,
            SortMethod::Size => SortMethod::Modified,
            SortMethod::Modified => SortMethod::Extension,
            SortMethod::Extension => SortMethod::Name,
        }
    }
}

#[derive(Debug)]
pub struct Tab {
    pub cwd: PathBuf,
    pub entries: Vec<Entry>,
    pub cursor: usize,
    pub selection: HashSet<usize>,
    pub scroll_offset: usize,
    pub show_hidden: bool,
    pub filter: Option<String>,
    pub search_query: Option<String>,
    pub search_matches: Vec<usize>,
    pub search_index: usize,
    pub sort_method: SortMethod,
    pub sort_reverse: bool,
    pub directories_first: bool,
}

impl Tab {
    pub fn new(cwd: PathBuf) -> Self {
        Self::with_show_hidden(cwd, false)
    }

    pub fn with_show_hidden(cwd: PathBuf, show_hidden: bool) -> Self {
        let mut tab = Self {
            cwd: cwd.clone(),
            entries: Vec::new(),
            cursor: 0,
            selection: HashSet::new(),
            scroll_offset: 0,
            show_hidden,
            filter: None,
            search_query: None,
            search_matches: Vec::new(),
            search_index: 0,
            sort_method: SortMethod::Name,
            sort_reverse: false,
            directories_first: true,
        };
        tab.reload();
        tab
    }

    pub fn reload(&mut self) {
        if let Ok(mut entries) = Entry::read_dir(&self.cwd) {
            if !self.show_hidden {
                entries.retain(|e| !e.is_hidden);
            }
            if let Some(ref filter) = self.filter {
                let filter_lower = filter.to_lowercase();
                entries.retain(|e| e.name.to_lowercase().contains(&filter_lower));
            }

            // ソート適用
            self.sort_entries(&mut entries);

            self.entries = entries;
            if self.cursor >= self.entries.len() && !self.entries.is_empty() {
                self.cursor = self.entries.len() - 1;
            }
        }
    }

    fn sort_entries(&self, entries: &mut Vec<Entry>) {
        entries.sort_by(|a, b| {
            // ディレクトリを最初に表示
            if self.directories_first {
                match (a.is_dir, b.is_dir) {
                    (true, false) => return std::cmp::Ordering::Less,
                    (false, true) => return std::cmp::Ordering::Greater,
                    _ => {}
                }
            }

            // ソート方法に応じて比較
            let ordering = match self.sort_method {
                SortMethod::Name => {
                    a.name.to_lowercase().cmp(&b.name.to_lowercase())
                }
                SortMethod::Size => {
                    a.size.cmp(&b.size)
                }
                SortMethod::Modified => {
                    a.modified.cmp(&b.modified)
                }
                SortMethod::Extension => {
                    let ext_a = std::path::Path::new(&a.name)
                        .extension()
                        .and_then(|s| s.to_str())
                        .unwrap_or("");
                    let ext_b = std::path::Path::new(&b.name)
                        .extension()
                        .and_then(|s| s.to_str())
                        .unwrap_or("");
                    ext_a.cmp(ext_b)
                }
            };

            if self.sort_reverse {
                ordering.reverse()
            } else {
                ordering
            }
        });
    }

    pub fn current_entry(&self) -> Option<&Entry> {
        self.entries.get(self.cursor)
    }

    pub fn move_cursor_down(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        // 最下端から下に移動した場合、最上端にループ
        self.cursor = (self.cursor + 1) % self.entries.len();
    }

    pub fn move_cursor_up(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        // 最上端から上に移動した場合、最下端にループ
        if self.cursor == 0 {
            self.cursor = self.entries.len() - 1;
        } else {
            self.cursor -= 1;
        }
    }

    /// スクロールオフセットを更新（画面内にカーソルを保持）
    pub fn update_scroll(&mut self, visible_lines: usize) {
        if visible_lines == 0 {
            return;
        }

        // カーソルが画面外（上）にある場合
        if self.cursor < self.scroll_offset {
            self.scroll_offset = self.cursor;
        }
        // カーソルが画面外（下）にある場合
        else if self.cursor >= self.scroll_offset + visible_lines {
            self.scroll_offset = self.cursor.saturating_sub(visible_lines - 1);
        }
    }

    pub fn enter(&mut self) -> bool {
        if let Some(entry) = self.current_entry() {
            if entry.is_dir {
                self.cwd = entry.path.clone();
                self.cursor = 0;
                self.scroll_offset = 0;
                self.reload();
                return true;
            }
        }
        false
    }

    pub fn parent(&mut self) -> bool {
        if let Some(parent) = self.cwd.parent() {
            // 移動前のディレクトリ名を保存
            let previous_dir = self.cwd.file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.to_string());

            self.cwd = parent.to_path_buf();
            self.cursor = 0;
            self.scroll_offset = 0;
            self.reload();

            // 元いたディレクトリを探してカーソルを合わせる
            if let Some(prev_name) = previous_dir {
                for (i, entry) in self.entries.iter().enumerate() {
                    if entry.name == prev_name {
                        self.cursor = i;
                        break;
                    }
                }
            }

            return true;
        }
        false
    }

    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.reload();
    }

    pub fn toggle_select(&mut self) {
        if !self.selection.remove(&self.cursor) {
            self.selection.insert(self.cursor);
        }
    }

    pub fn clear_selection(&mut self) {
        self.selection.clear();
    }

    pub fn is_selected(&self, index: usize) -> bool {
        self.selection.contains(&index)
    }

    pub fn get_selected_entries(&self) -> Vec<&Entry> {
        self.selection
            .iter()
            .filter_map(|&i| self.entries.get(i))
            .collect()
    }

    pub fn get_selected_paths(&self) -> Vec<PathBuf> {
        self.get_selected_entries()
            .iter()
            .map(|e| e.path.clone())
            .collect()
    }

    pub fn set_filter(&mut self, filter: Option<String>) {
        self.filter = filter;
        self.reload();
    }

    pub fn cycle_sort_method(&mut self) {
        self.sort_method = self.sort_method.next();
        self.reload();
    }

    pub fn toggle_sort_reverse(&mut self) {
        self.sort_reverse = !self.sort_reverse;
        self.reload();
    }

    pub fn toggle_directories_first(&mut self) {
        self.directories_first = !self.directories_first;
        self.reload();
    }

    pub fn search(&mut self, query: &str) {
        self.search_query = Some(query.to_string());
        self.search_matches.clear();
        self.search_index = 0;

        let query_lower = query.to_lowercase();
        for (i, entry) in self.entries.iter().enumerate() {
            if entry.name.to_lowercase().contains(&query_lower) {
                self.search_matches.push(i);
            }
        }

        if !self.search_matches.is_empty() {
            self.cursor = self.search_matches[0];
        }
    }

    pub fn search_next(&mut self) {
        if self.search_matches.is_empty() {
            return;
        }
        self.search_index = (self.search_index + 1) % self.search_matches.len();
        self.cursor = self.search_matches[self.search_index];
    }

    pub fn search_prev(&mut self) {
        if self.search_matches.is_empty() {
            return;
        }
        if self.search_index == 0 {
            self.search_index = self.search_matches.len() - 1;
        } else {
            self.search_index -= 1;
        }
        self.cursor = self.search_matches[self.search_index];
    }

    pub fn clear_search(&mut self) {
        self.search_query = None;
        self.search_matches.clear();
        self.search_index = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_sort_method_cycle() {
        assert_eq!(SortMethod::Name.next(), SortMethod::Size);
        assert_eq!(SortMethod::Size.next(), SortMethod::Modified);
        assert_eq!(SortMethod::Modified.next(), SortMethod::Extension);
        assert_eq!(SortMethod::Extension.next(), SortMethod::Name);
    }

    #[test]
    fn test_tab_creation() {
        let cwd = env::current_dir().unwrap();
        let tab = Tab::new(cwd.clone());

        assert_eq!(tab.cwd, cwd);
        assert_eq!(tab.cursor, 0);
        assert_eq!(tab.show_hidden, false);
        assert_eq!(tab.sort_method, SortMethod::Name);
        assert_eq!(tab.sort_reverse, false);
        assert_eq!(tab.directories_first, true);
    }

    #[test]
    fn test_tab_with_show_hidden() {
        let cwd = env::current_dir().unwrap();
        let tab = Tab::with_show_hidden(cwd.clone(), true);

        assert_eq!(tab.show_hidden, true);
    }

    #[test]
    fn test_cursor_movement() {
        let cwd = env::current_dir().unwrap();
        let mut tab = Tab::new(cwd);

        if !tab.entries.is_empty() {
            let initial_cursor = tab.cursor;

            // 下に移動
            tab.move_cursor_down();
            if tab.entries.len() > 1 {
                assert_eq!(tab.cursor, initial_cursor + 1);
            } else {
                assert_eq!(tab.cursor, 0); // 1つしかない場合はループして0に戻る
            }

            // 上に移動
            tab.move_cursor_up();
            assert_eq!(tab.cursor, initial_cursor);

            // 0番目で上に移動するとループして最後に移動
            tab.move_cursor_up();
            assert_eq!(tab.cursor, tab.entries.len() - 1);

            // 最下端で下に移動するとループして最初に戻る
            tab.cursor = tab.entries.len() - 1;
            tab.move_cursor_down();
            assert_eq!(tab.cursor, 0);
        }
    }

    #[test]
    fn test_selection() {
        let cwd = env::current_dir().unwrap();
        let mut tab = Tab::new(cwd);

        if !tab.entries.is_empty() {
            // 選択を追加
            assert!(!tab.is_selected(0));
            tab.toggle_select();
            assert!(tab.is_selected(0));

            // 選択を解除
            tab.toggle_select();
            assert!(!tab.is_selected(0));

            // 複数選択
            tab.toggle_select();
            if tab.entries.len() > 1 {
                tab.cursor = 1;
                tab.toggle_select();
                assert_eq!(tab.selection.len(), 2);
            }

            // 選択をクリア
            tab.clear_selection();
            assert_eq!(tab.selection.len(), 0);
        }
    }

    #[test]
    fn test_toggle_hidden() {
        let cwd = env::current_dir().unwrap();
        let mut tab = Tab::new(cwd);

        let initial_hidden = tab.show_hidden;
        tab.toggle_hidden();
        assert_eq!(tab.show_hidden, !initial_hidden);

        tab.toggle_hidden();
        assert_eq!(tab.show_hidden, initial_hidden);
    }

    #[test]
    fn test_sort_method_cycle_in_tab() {
        let cwd = env::current_dir().unwrap();
        let mut tab = Tab::new(cwd);

        assert_eq!(tab.sort_method, SortMethod::Name);

        tab.cycle_sort_method();
        assert_eq!(tab.sort_method, SortMethod::Size);

        tab.cycle_sort_method();
        assert_eq!(tab.sort_method, SortMethod::Modified);

        tab.cycle_sort_method();
        assert_eq!(tab.sort_method, SortMethod::Extension);

        tab.cycle_sort_method();
        assert_eq!(tab.sort_method, SortMethod::Name);
    }

    #[test]
    fn test_sort_reverse() {
        let cwd = env::current_dir().unwrap();
        let mut tab = Tab::new(cwd);

        assert_eq!(tab.sort_reverse, false);

        tab.toggle_sort_reverse();
        assert_eq!(tab.sort_reverse, true);

        tab.toggle_sort_reverse();
        assert_eq!(tab.sort_reverse, false);
    }

    #[test]
    fn test_filter() {
        let cwd = env::current_dir().unwrap();
        let mut tab = Tab::new(cwd);

        let original_count = tab.entries.len();

        // フィルター設定
        tab.set_filter(Some("test".to_string()));
        assert!(tab.filter.is_some());

        // フィルターされるとエントリ数が変わる可能性がある
        let filtered_count = tab.entries.len();
        assert!(filtered_count <= original_count);

        // フィルター解除
        tab.set_filter(None);
        assert!(tab.filter.is_none());
        assert_eq!(tab.entries.len(), original_count);
    }

    #[test]
    fn test_search() {
        let cwd = env::current_dir().unwrap();
        let mut tab = Tab::new(cwd);

        if !tab.entries.is_empty() {
            // 最初のエントリの名前の一部で検索
            let query = tab.entries[0].name[0..1.min(tab.entries[0].name.len())].to_string();

            tab.search(&query);

            assert!(tab.search_query.is_some());
            assert!(!tab.search_matches.is_empty());

            // 検索結果ナビゲーション
            if tab.search_matches.len() > 1 {
                let first_match = tab.search_matches[0];
                tab.search_next();
                assert_eq!(tab.cursor, tab.search_matches[1]);

                tab.search_prev();
                assert_eq!(tab.cursor, first_match);
            }

            // 検索クリア
            tab.clear_search();
            assert!(tab.search_query.is_none());
            assert!(tab.search_matches.is_empty());
        }
    }

    #[test]
    fn test_get_selected_entries() {
        let cwd = env::current_dir().unwrap();
        let mut tab = Tab::new(cwd);

        if tab.entries.len() >= 2 {
            tab.cursor = 0;
            tab.toggle_select();
            tab.cursor = 1;
            tab.toggle_select();

            let selected = tab.get_selected_entries();
            assert_eq!(selected.len(), 2);

            let paths = tab.get_selected_paths();
            assert_eq!(paths.len(), 2);
        }
    }

    #[test]
    fn test_parent_focuses_previous_dir() {
        let cwd = env::current_dir().unwrap();

        // 親ディレクトリが存在する場合のみテスト
        if cwd.parent().is_some() {
            let mut tab = Tab::new(cwd.clone());

            // カレントディレクトリのディレクトリ名を取得
            let current_dir_name = cwd.file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.to_string());

            // 親ディレクトリに移動
            let moved = tab.parent();
            assert!(moved);

            // カーソルが元いたディレクトリに合っているか確認
            if let Some(dir_name) = current_dir_name {
                if let Some(entry) = tab.current_entry() {
                    assert_eq!(entry.name, dir_name);
                }
            }
        }
    }
}
