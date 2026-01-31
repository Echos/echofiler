use crate::config::watcher::ConfigWatcher;
use crate::config::Config;
use crate::core::{BookmarkList, Clipboard, ClipboardMode, Pane, PaneSide};
use crate::fs::watcher::FileWatcher;
use crate::input::{InputMode, Key};
use crossterm::event::{KeyCode, KeyModifiers};
use std::env;
use std::path::PathBuf;

use super::action::Action;
use super::action_parser::parse_action;
use super::confirm::PendingAction;

pub struct App {
    pub left_pane: Pane,
    pub right_pane: Pane,
    pub active_pane: PaneSide,
    pub mode: InputMode,
    pub config: Config,
    pub clipboard: Clipboard,
    pub command_input: String,
    pub command_prompt: String,
    pub show_preview: bool,
    pub bookmarks: BookmarkList,
    pub bookmark_cursor: usize,
    pub file_watcher: Option<FileWatcher>,
    pub config_watcher: Option<ConfigWatcher>,
    pub pending_action: Option<PendingAction>,
    pub confirm_message: String,
    pub status_message: String,
    pub dialog_message: String,
    pub is_error_dialog: bool,
    pub suspend_for_command: Option<(String, std::path::PathBuf)>, // (command, path)
    pub current_prefix: Option<String>,  // 現在のプレフィックスキー（特殊ワード）
    pub screen_needs_clear: bool,  // 画面をクリアする必要があるか
    pub should_quit: bool,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let cwd = env::current_dir()?;
        let config = Config::load()?;
        let show_hidden = config.general.show_hidden;
        let bookmarks = BookmarkList::load().unwrap_or_default();

        // ファイル監視機能を初期化（失敗してもcontinue）
        let mut file_watcher = FileWatcher::new().ok();
        if let Some(ref mut watcher) = file_watcher {
            let _ = watcher.watch(&cwd);
        }

        // 設定ファイル監視機能を初期化
        let mut config_watcher = ConfigWatcher::new().ok();
        if let Some(ref mut watcher) = config_watcher {
            let _ = watcher.watch(&Config::get_config_path());
            let _ = watcher.watch(&Config::get_theme_path());
            let _ = watcher.watch(&Config::get_keymap_path());
        }

        Ok(Self {
            left_pane: Pane::with_show_hidden(cwd.clone(), show_hidden),
            right_pane: Pane::with_show_hidden(cwd, show_hidden),
            active_pane: PaneSide::Left,
            mode: InputMode::Normal,
            config,
            clipboard: Clipboard::default(),
            command_input: String::new(),
            command_prompt: String::new(),
            show_preview: false,
            bookmarks,
            bookmark_cursor: 0,
            file_watcher,
            config_watcher,
            pending_action: None,
            confirm_message: String::new(),
            status_message: String::new(),
            dialog_message: String::new(),
            is_error_dialog: false,
            suspend_for_command: None,
            current_prefix: None,
            screen_needs_clear: false,
            should_quit: false,
        })
    }

    pub fn active_pane(&self) -> &Pane {
        match self.active_pane {
            PaneSide::Left => &self.left_pane,
            PaneSide::Right => &self.right_pane,
        }
    }

    pub fn active_pane_mut(&mut self) -> &mut Pane {
        match self.active_pane {
            PaneSide::Left => &mut self.left_pane,
            PaneSide::Right => &mut self.right_pane,
        }
    }

    pub fn handle_key(&mut self, key: Key) -> Action {
        use crate::config::keymap::KeymapConfig;

        // 前回のステータスメッセージをクリア（Confirm/MessageDialogモードを除く）
        if !matches!(self.mode, InputMode::Confirm | InputMode::MessageDialog) {
            self.status_message.clear();
        }

        // プレフィックスモード中の場合、プレフィックス後のキーマップをチェック
        if self.mode == InputMode::BookmarkPrefix {
            if let Some(ref prefix) = self.current_prefix {
                let mode_str = "normal";  // プレフィックスはNormalモードから入る
                let key_str = key.to_keymap_string();
                if !key_str.is_empty() {
                    if let Some(action_name) = self.config.keymap.get_prefix_action(mode_str, prefix, &key_str) {
                        // プレフィックスをクリア
                        self.current_prefix = None;
                        self.mode = InputMode::Normal;
                        if let Some(action) = parse_action(action_name) {
                            return action;
                        }
                    }
                }
            }
            // プレフィックス後のキーマップにない場合はデフォルト動作へ
        }

        // キーマップをチェック（Bookmark/BookmarkPrefix/BookmarkSelect/Help/Confirm/MessageDialogモード以外）
        if !matches!(self.mode, InputMode::Bookmark | InputMode::BookmarkPrefix | InputMode::BookmarkSelect | InputMode::Help | InputMode::Confirm | InputMode::MessageDialog) {
            let mode_str = match self.mode {
                InputMode::Normal => "normal",
                InputMode::Visual => "visual",
                InputMode::Command => "command",
                InputMode::Search => "search",
                InputMode::Bookmark | InputMode::BookmarkPrefix | InputMode::BookmarkSelect | InputMode::Help | InputMode::Confirm | InputMode::MessageDialog => unreachable!(),
            };

            let key_str = key.to_keymap_string();
            if !key_str.is_empty() {
                if let Some(action_name) = self.config.keymap.get_action(mode_str, &key_str) {
                    // 特殊ワードかどうかをチェック
                    if KeymapConfig::is_special_word(action_name) {
                        // プレフィックスキーとして扱う
                        self.current_prefix = Some(action_name.to_string());
                        self.mode = InputMode::BookmarkPrefix;
                        return Action::None;
                    }

                    if let Some(action) = parse_action(action_name) {
                        return action;
                    }
                }
            }
        }

        // キーマップにない場合はデフォルトの動作
        match self.mode {
            InputMode::Normal => self.handle_normal_mode(key),
            InputMode::Visual => self.handle_visual_mode(key),
            InputMode::Command => self.handle_command_mode(key),
            InputMode::Search => self.handle_search_mode(key),
            InputMode::Bookmark => self.handle_bookmark_mode(key),
            InputMode::BookmarkPrefix => self.handle_bookmark_prefix_mode(key),
            InputMode::BookmarkSelect => self.handle_bookmark_select_mode(key),
            InputMode::Help => self.handle_help_mode(key),
            InputMode::Confirm => self.handle_confirm_mode(key),
            InputMode::MessageDialog => self.handle_message_dialog_mode(key),
        }
    }

    fn handle_normal_mode(&mut self, key: Key) -> Action {
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('t') => return Action::NewTab,
                KeyCode::Char('w') => return Action::CloseTab,
                KeyCode::Char('l') => return Action::ScreenRefresh,
                _ => {}
            }
        }

        match key.code {
            KeyCode::Char('q') => Action::Quit,
            KeyCode::Char('j') | KeyCode::Down => Action::CursorDown,
            KeyCode::Char('k') | KeyCode::Up => Action::CursorUp,
            KeyCode::Enter => Action::Enter,
            KeyCode::Backspace => Action::Parent,
            KeyCode::Char('h') => Action::PrevTab,
            KeyCode::Char('l') => Action::NextTab,
            KeyCode::Left => Action::FocusLeft,
            KeyCode::Right => Action::FocusRight,
            KeyCode::Char('.') => Action::ToggleHidden,
            KeyCode::Tab => Action::TogglePane,
            KeyCode::Char('r') => Action::Refresh,
            KeyCode::Char(' ') => Action::ToggleSelect,
            KeyCode::Char('v') => Action::VisualMode,
            KeyCode::Char('y') => Action::Yank,
            KeyCode::Char('d') => Action::Cut,
            KeyCode::Char('p') => Action::Paste,
            KeyCode::Char('D') => Action::Delete,
            KeyCode::Char('R') => Action::Rename,
            KeyCode::Char('a') => Action::Create,
            KeyCode::Char('/') => Action::SearchMode,
            KeyCode::Char('f') => Action::FilterMode,
            KeyCode::Char('n') => Action::SearchNext,
            KeyCode::Char('N') => Action::SearchPrev,
            KeyCode::Char('[') => Action::PrevTab,
            KeyCode::Char(']') => Action::NextTab,
            KeyCode::Char('P') => Action::TogglePreview,
            KeyCode::Char('s') => Action::CycleSortMethod,
            KeyCode::Char('S') => Action::ToggleSortReverse,
            KeyCode::Char('i') => Action::ToggleSidebar,
            KeyCode::Char('?') => Action::ShowHelp,
            KeyCode::Char('e') => Action::ExtractArchive,
            KeyCode::Char('z') => Action::CompressToZip,
            KeyCode::Char('o') => Action::OpenFile,
            KeyCode::Char('C') => Action::CopyToOtherPane,
            KeyCode::Char('M') => Action::MoveToOtherPane,
            KeyCode::Char('E') => Action::OpenWithEditor,
            KeyCode::Char('w') => Action::OpenWithPager,
            KeyCode::Char('X') => Action::ExecuteFile,
            KeyCode::Char(':') => {
                self.command_prompt = "".to_string();
                self.command_input = ":".to_string();
                self.mode = InputMode::Command;
                Action::None
            }
            _ => Action::None,
        }
    }

    fn handle_visual_mode(&mut self, key: Key) -> Action {
        match key.code {
            KeyCode::Esc => Action::NormalMode,
            KeyCode::Char('j') | KeyCode::Down => Action::CursorDown,
            KeyCode::Char('k') | KeyCode::Up => Action::CursorUp,
            KeyCode::Char('y') => Action::Yank,
            KeyCode::Char('d') => Action::Cut,
            KeyCode::Char('D') => Action::Delete,
            _ => Action::None,
        }
    }

    fn handle_command_mode(&mut self, key: Key) -> Action {
        match key.code {
            KeyCode::Esc => {
                self.command_input.clear();
                self.status_message.clear();
                Action::NormalMode
            }
            KeyCode::Enter => {
                let input = self.command_input.clone();
                self.command_input.clear();
                self.execute_command(&input);
                Action::NormalMode
            }
            KeyCode::Backspace => {
                self.command_input.pop();
                self.status_message.clear();
                Action::None
            }
            KeyCode::Tab => {
                // コマンド補完
                if self.command_input.starts_with(':') {
                    self.complete_command();
                }
                Action::None
            }
            KeyCode::Char(c) => {
                self.command_input.push(c);
                self.status_message.clear();
                Action::None
            }
            _ => Action::None,
        }
    }

    fn handle_search_mode(&mut self, key: Key) -> Action {
        match key.code {
            KeyCode::Esc => {
                self.command_input.clear();
                self.active_pane_mut().current_tab_mut().clear_search();
                Action::NormalMode
            }
            KeyCode::Enter => {
                let query = self.command_input.clone();
                self.command_input.clear();
                if !query.is_empty() {
                    self.active_pane_mut().current_tab_mut().search(&query);
                }
                Action::NormalMode
            }
            KeyCode::Backspace => {
                self.command_input.pop();
                let query = self.command_input.clone();
                if query.is_empty() {
                    self.active_pane_mut().current_tab_mut().clear_search();
                } else {
                    self.active_pane_mut().current_tab_mut().search(&query);
                }
                Action::None
            }
            KeyCode::Char(c) => {
                self.command_input.push(c);
                let query = self.command_input.clone();
                self.active_pane_mut().current_tab_mut().search(&query);
                Action::None
            }
            _ => Action::None,
        }
    }

    fn handle_bookmark_mode(&mut self, key: Key) -> Action {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => Action::NormalMode,
            KeyCode::Char('j') | KeyCode::Down => {
                if self.bookmark_cursor < self.bookmarks.bookmarks.len().saturating_sub(1) {
                    self.bookmark_cursor += 1;
                }
                Action::None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.bookmark_cursor > 0 {
                    self.bookmark_cursor -= 1;
                }
                Action::None
            }
            KeyCode::Enter => Action::JumpToBookmark,
            KeyCode::Char('d') | KeyCode::Delete => Action::DeleteBookmark,
            KeyCode::Char(c) => {
                // j, k, q, d以外の文字キーでブックマークへジャンプ
                if c != 'j' && c != 'k' && c != 'q' && c != 'd' {
                    if let Some(bookmark) = self.bookmarks.find_by_key(c) {
                        let path = bookmark.path.clone();
                        self.active_pane_mut().current_tab_mut().cwd = path;
                        self.active_pane_mut().current_tab_mut().reload();
                        self.mode = InputMode::Normal;
                        return Action::None;
                    } else {
                        self.status_message = format!("No bookmark for key: {}", c);
                    }
                }
                Action::None
            }
            _ => Action::None,
        }
    }

    fn handle_help_mode(&mut self, key: Key) -> Action {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?') => Action::NormalMode,
            _ => Action::None,
        }
    }

    fn handle_bookmark_select_mode(&mut self, key: Key) -> Action {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => Action::NormalMode,
            KeyCode::Char(c) => {
                // キーに対応するブックマークを検索
                if let Some(bookmark) = self.bookmarks.find_by_key(c) {
                    let path = bookmark.path.clone();
                    self.active_pane_mut().current_tab_mut().cwd = path;
                    self.active_pane_mut().current_tab_mut().reload();
                    self.mode = InputMode::Normal;
                    Action::None
                } else {
                    // 対応するブックマークがない場合
                    self.status_message = format!("No bookmark for key: {}", c);
                    self.mode = InputMode::Normal;
                    Action::None
                }
            }
            _ => Action::None,
        }
    }

    fn handle_bookmark_prefix_mode(&mut self, key: Key) -> Action {
        // プレフィックスをクリア
        self.current_prefix = None;

        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.mode = InputMode::Normal;
                Action::None
            }
            KeyCode::Char(c) => {
                // デフォルト動作: そのキーに対応するブックマークへジャンプ
                // キーマップで定義されている場合は、handle_key()で処理済み
                if let Some(bookmark) = self.bookmarks.find_by_key(c) {
                    let path = bookmark.path.clone();
                    self.active_pane_mut().current_tab_mut().cwd = path;
                    self.active_pane_mut().current_tab_mut().reload();
                    self.mode = InputMode::Normal;
                    Action::None
                } else {
                    self.status_message = format!("No bookmark for key: {}", c);
                    self.mode = InputMode::Normal;
                    Action::None
                }
            }
            _ => {
                self.mode = InputMode::Normal;
                Action::None
            }
        }
    }

    fn handle_message_dialog_mode(&mut self, _key: Key) -> Action {
        // 任意のキーでダイアログを閉じる
        self.dialog_message.clear();
        Action::NormalMode
    }

    fn handle_confirm_mode(&mut self, key: Key) -> Action {
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                // 確認された操作を実行
                self.execute_pending_action();
                self.mode = InputMode::Normal;
                Action::None
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                // キャンセル
                self.pending_action = None;
                self.confirm_message.clear();
                self.mode = InputMode::Normal;
                Action::None
            }
            _ => Action::None,
        }
    }

    fn execute_pending_action(&mut self) {
        if let Some(action) = self.pending_action.take() {
            match action {
                PendingAction::Delete { paths } => {
                    let mut errors = Vec::new();
                    for path in &paths {
                        if let Err(e) = crate::fs::ops::delete_file(path) {
                            errors.push(format!("{}: {}", path.display(), e));
                        }
                    }
                    self.active_pane_mut().current_tab_mut().clear_selection();
                    self.active_pane_mut().current_tab_mut().reload();

                    if !errors.is_empty() {
                        let error_msg = if errors.len() == 1 {
                            format!("Failed to delete file:\n{}", errors[0])
                        } else {
                            format!("Failed to delete {} files:\n{}", errors.len(), errors.join("\n"))
                        };
                        self.show_error(&error_msg);
                    }
                }
                PendingAction::Paste { .. } => {
                    self.execute_paste();
                }
                PendingAction::ExtractArchive { archive_path, dest_dir } => {
                    #[cfg(feature = "archive")]
                    {
                        use crate::fs::archive::extract_archive;
                        match extract_archive(&archive_path, &dest_dir) {
                            Ok(_) => {
                                self.active_pane_mut().current_tab_mut().reload();
                                self.show_message(&format!("Extracted: {}", archive_path.display()));
                            }
                            Err(e) => {
                                self.show_error(&format!("Failed to extract archive:\n{}", e));
                            }
                        }
                    }
                    #[cfg(not(feature = "archive"))]
                    {
                        let _ = (archive_path, dest_dir);
                        self.show_error("Archive support requires 'archive' feature");
                    }
                }
                PendingAction::CopyToOtherPane { paths, dest_dir } => {
                    self.execute_copy_to_other_pane(paths, dest_dir);
                }
                PendingAction::MoveToOtherPane { paths, dest_dir } => {
                    self.execute_move_to_other_pane(paths, dest_dir);
                }
                PendingAction::Quit => {
                    self.should_quit = true;
                }
            }
        }
        self.confirm_message.clear();
    }

    pub fn update(&mut self, action: Action) {
        match action {
            Action::Quit => {
                if self.config.general.confirm_quit {
                    self.pending_action = Some(PendingAction::Quit);
                    self.mode = InputMode::Confirm;
                } else {
                    self.should_quit = true;
                }
            }
            Action::CursorDown => {
                self.active_pane_mut().current_tab_mut().move_cursor_down();
                if self.mode == InputMode::Visual {
                    self.active_pane_mut().current_tab_mut().toggle_select();
                }
            }
            Action::CursorUp => {
                self.active_pane_mut().current_tab_mut().move_cursor_up();
                if self.mode == InputMode::Visual {
                    self.active_pane_mut().current_tab_mut().toggle_select();
                }
            }
            Action::Enter => {
                // カーソル下のエントリを取得
                if let Some(entry) = self.active_pane().current_tab().current_entry() {
                    if entry.is_dir {
                        // ディレクトリの場合は入る
                        self.active_pane_mut().current_tab_mut().enter();
                    } else {
                        // ファイルの場合は開く
                        let path = entry.path.clone();
                        if let Err(e) = crate::fs::opener::open_file(&path, &self.config.opener) {
                            self.show_error(&format!("Failed to open file:\n{}", e));
                        }
                    }
                }
            }
            Action::Parent => {
                self.active_pane_mut().current_tab_mut().parent();
            }
            Action::ToggleHidden => {
                self.active_pane_mut().current_tab_mut().toggle_hidden();
            }
            Action::TogglePane => {
                self.active_pane = match self.active_pane {
                    PaneSide::Left => PaneSide::Right,
                    PaneSide::Right => PaneSide::Left,
                };
            }
            Action::FocusLeft => {
                self.active_pane = PaneSide::Left;
            }
            Action::FocusRight => {
                self.active_pane = PaneSide::Right;
            }
            Action::Refresh => {
                self.left_pane.current_tab_mut().reload();
                self.right_pane.current_tab_mut().reload();
            }
            Action::ScreenRefresh => {
                // 画面をクリアしてから再描画
                self.screen_needs_clear = true;
                self.left_pane.current_tab_mut().reload();
                self.right_pane.current_tab_mut().reload();
            }
            Action::ToggleSelect => {
                let tab = self.active_pane_mut().current_tab_mut();
                tab.toggle_select();
                tab.move_cursor_down();
            }
            Action::VisualMode => {
                self.mode = InputMode::Visual;
                self.active_pane_mut().current_tab_mut().toggle_select();
            }
            Action::NormalMode => {
                self.mode = InputMode::Normal;
            }
            Action::Yank => {
                self.yank_files();
            }
            Action::Cut => {
                self.cut_files();
            }
            Action::Paste => {
                self.paste_files();
            }
            Action::Delete => {
                self.delete_files();
            }
            Action::CopyToOtherPane => {
                self.copy_to_other_pane();
            }
            Action::MoveToOtherPane => {
                self.move_to_other_pane();
            }
            Action::Rename => {
                let name = self.active_pane()
                    .current_tab()
                    .current_entry()
                    .map(|e| e.name.clone());
                if let Some(name) = name {
                    self.command_prompt = "Rename to: ".to_string();
                    self.command_input = name;
                    self.mode = InputMode::Command;
                }
            }
            Action::Create => {
                self.command_prompt = "Create (file/dir): ".to_string();
                self.command_input.clear();
                self.mode = InputMode::Command;
            }
            Action::SearchMode => {
                self.command_prompt = "Search: ".to_string();
                self.command_input.clear();
                self.mode = InputMode::Search;
            }
            Action::FilterMode => {
                self.command_prompt = "Filter: ".to_string();
                self.command_input.clear();
                self.mode = InputMode::Command;
            }
            Action::SearchNext => {
                self.active_pane_mut().current_tab_mut().search_next();
            }
            Action::SearchPrev => {
                self.active_pane_mut().current_tab_mut().search_prev();
            }
            Action::NewTab => {
                let cwd = self.active_pane().current_tab().cwd.clone();
                let show_hidden = self.config.general.show_hidden;
                self.active_pane_mut().new_tab(cwd, show_hidden);
            }
            Action::CloseTab => {
                self.active_pane_mut().close_tab();
            }
            Action::NextTab => {
                self.active_pane_mut().next_tab();
            }
            Action::PrevTab => {
                self.active_pane_mut().prev_tab();
            }
            Action::TogglePreview => {
                self.show_preview = !self.show_preview;
            }
            Action::CycleSortMethod => {
                self.active_pane_mut().current_tab_mut().cycle_sort_method();
            }
            Action::ToggleSortReverse => {
                self.active_pane_mut().current_tab_mut().toggle_sort_reverse();
            }
            Action::ToggleSidebar => {
                self.config.sidebar.enabled = !self.config.sidebar.enabled;
            }
            Action::AddBookmark => {
                self.command_prompt = "Bookmark (name [key]): ".to_string();
                self.command_input.clear();
                self.mode = InputMode::Command;
            }
            Action::ShowBookmarks => {
                self.bookmark_cursor = 0;
                self.mode = InputMode::Bookmark;
            }
            Action::ShowBookmarkSelect => {
                self.mode = InputMode::BookmarkSelect;
            }
            Action::DeleteBookmark => {
                if self.bookmark_cursor < self.bookmarks.bookmarks.len() {
                    self.bookmarks.remove(self.bookmark_cursor);
                    let _ = self.bookmarks.save();
                    if self.bookmark_cursor >= self.bookmarks.bookmarks.len() && self.bookmark_cursor > 0 {
                        self.bookmark_cursor -= 1;
                    }
                }
            }
            Action::JumpToBookmark => {
                if let Some(bookmark) = self.bookmarks.bookmarks.get(self.bookmark_cursor) {
                    let path = bookmark.path.clone();
                    self.active_pane_mut().current_tab_mut().cwd = path.clone();
                    self.active_pane_mut().current_tab_mut().reload();
                    self.mode = InputMode::Normal;
                }
            }
            Action::ShowHelp => {
                self.mode = InputMode::Help;
            }
            Action::ExtractArchive => {
                self.extract_archive();
            }
            Action::CompressToZip => {
                self.compress_to_zip();
            }
            Action::OpenFile => {
                self.open_current_file();
            }
            Action::OpenWithEditor => {
                self.open_with_editor();
            }
            Action::OpenWithPager => {
                self.open_with_pager();
            }
            Action::ExecuteFile => {
                self.execute_current_file();
            }
            Action::EditConfig => {
                self.edit_config_file();
            }
            Action::EditKeymap => {
                self.edit_keymap_file();
            }
            Action::EditTheme => {
                self.edit_theme_file();
            }
            Action::EditOpener => {
                self.edit_opener_file();
            }
            Action::None => {}
        }
    }

    pub fn check_file_changes(&mut self) -> bool {
        if let Some(ref watcher) = self.file_watcher {
            if watcher.check_events() {
                watcher.clear_events();
                return true;
            }
        }
        false
    }

    pub fn check_config_changes(&mut self) -> bool {
        if let Some(ref watcher) = self.config_watcher {
            if watcher.check_changes() {
                watcher.clear_events();
                // 設定ファイルを再読み込み
                if let Ok(new_config) = Config::load() {
                    self.config = new_config;
                    return true;
                }
            }
        }
        false
    }

    pub fn update_watch_path(&mut self) {
        let path = self.active_pane().current_tab().cwd.clone();
        if let Some(ref mut watcher) = self.file_watcher {
            let _ = watcher.watch(&path);
        }
    }

    fn yank_files(&mut self) {
        let paths = {
            let tab = self.active_pane_mut().current_tab_mut();
            if tab.selection.is_empty() {
                if let Some(entry) = tab.current_entry() {
                    vec![entry.path.clone()]
                } else {
                    return;
                }
            } else {
                tab.get_selected_paths()
            }
        };

        self.clipboard.paths = paths;
        self.clipboard.mode = Some(ClipboardMode::Copy);

        self.active_pane_mut().current_tab_mut().clear_selection();
        self.mode = InputMode::Normal;
    }

    fn cut_files(&mut self) {
        let paths = {
            let tab = self.active_pane_mut().current_tab_mut();
            if tab.selection.is_empty() {
                if let Some(entry) = tab.current_entry() {
                    vec![entry.path.clone()]
                } else {
                    return;
                }
            } else {
                tab.get_selected_paths()
            }
        };

        self.clipboard.paths = paths;
        self.clipboard.mode = Some(ClipboardMode::Cut);

        self.active_pane_mut().current_tab_mut().clear_selection();
        self.mode = InputMode::Normal;
    }

    fn paste_files(&mut self) {
        if self.clipboard.is_empty() {
            return;
        }

        let dest_dir = self.active_pane().current_tab().cwd.clone();

        // 上書きの可能性をチェック
        let has_conflicts = self.clipboard.paths.iter().any(|src| {
            if let Some(file_name) = src.file_name() {
                dest_dir.join(file_name).exists()
            } else {
                false
            }
        });

        // 上書き確認が必要な場合
        if has_conflicts && self.config.general.confirm_overwrite {
            let action = PendingAction::Paste { has_conflicts };
            self.confirm_message = action.message();
            self.pending_action = Some(action);
            self.mode = InputMode::Confirm;
            return;
        }

        // 確認なしで即実行
        self.execute_paste();
    }

    fn execute_paste(&mut self) {
        let dest_dir = self.active_pane().current_tab().cwd.clone();
        let mut errors = Vec::new();

        match &self.clipboard.mode {
            Some(ClipboardMode::Copy) => {
                for src in &self.clipboard.paths {
                    if let Some(file_name) = src.file_name() {
                        let dest = dest_dir.join(file_name);
                        if let Err(e) = crate::fs::ops::copy_file(src, &dest) {
                            errors.push(format!("{}: {}", src.display(), e));
                        }
                    }
                }
            }
            Some(ClipboardMode::Cut) => {
                for src in &self.clipboard.paths {
                    if let Some(file_name) = src.file_name() {
                        let dest = dest_dir.join(file_name);
                        if let Err(e) = crate::fs::ops::move_file(src, &dest) {
                            errors.push(format!("{}: {}", src.display(), e));
                        }
                    }
                }
                self.clipboard.clear();
            }
            None => {}
        }

        self.active_pane_mut().current_tab_mut().reload();

        if !errors.is_empty() {
            let error_msg = if errors.len() == 1 {
                format!("Failed to paste file:\n{}", errors[0])
            } else {
                format!("Failed to paste {} files:\n{}", errors.len(), errors.join("\n"))
            };
            self.show_error(&error_msg);
        }
    }

    fn copy_to_other_pane(&mut self) {
        // 選択されたファイル、またはカーソル下のファイルを取得
        let tab = self.active_pane().current_tab();
        let paths = if tab.selection.is_empty() {
            if let Some(entry) = tab.current_entry() {
                vec![entry.path.clone()]
            } else {
                self.show_message("No file selected");
                return;
            }
        } else {
            tab.get_selected_paths()
        };

        // 逆ペインのカレントディレクトリを取得
        let dest_dir = match self.active_pane {
            PaneSide::Left => self.right_pane.current_tab().cwd.clone(),
            PaneSide::Right => self.left_pane.current_tab().cwd.clone(),
        };

        // 上書きの可能性をチェック
        let has_conflicts = paths.iter().any(|src| {
            if let Some(file_name) = src.file_name() {
                dest_dir.join(file_name).exists()
            } else {
                false
            }
        });

        // 上書き確認が必要な場合
        if has_conflicts && self.config.general.confirm_overwrite {
            let action = PendingAction::CopyToOtherPane { paths, dest_dir };
            self.confirm_message = action.message();
            self.pending_action = Some(action);
            self.mode = InputMode::Confirm;
            return;
        }

        // 確認なしで即実行
        self.execute_copy_to_other_pane(paths, dest_dir);
    }

    fn execute_copy_to_other_pane(&mut self, paths: Vec<PathBuf>, dest_dir: PathBuf) {
        let mut success_count = 0;
        let mut errors = Vec::new();

        for src in &paths {
            if let Some(file_name) = src.file_name() {
                let dest = dest_dir.join(file_name);
                match crate::fs::ops::copy_file(src, &dest) {
                    Ok(_) => success_count += 1,
                    Err(e) => errors.push(format!("{}: {}", src.display(), e)),
                }
            }
        }

        // 両ペインをリロード
        self.left_pane.current_tab_mut().reload();
        self.right_pane.current_tab_mut().reload();

        // 選択をクリア
        self.active_pane_mut().current_tab_mut().clear_selection();

        // メッセージ表示
        if errors.is_empty() {
            self.show_message(&format!("Copied {} item(s) to other pane", success_count));
        } else {
            let error_msg = format!(
                "Copied {} item(s), {} error(s):\n{}",
                success_count,
                errors.len(),
                errors.join("\n")
            );
            self.show_error(&error_msg);
        }
    }

    fn move_to_other_pane(&mut self) {
        // 選択されたファイル、またはカーソル下のファイルを取得
        let tab = self.active_pane().current_tab();
        let paths = if tab.selection.is_empty() {
            if let Some(entry) = tab.current_entry() {
                vec![entry.path.clone()]
            } else {
                self.show_message("No file selected");
                return;
            }
        } else {
            tab.get_selected_paths()
        };

        // 逆ペインのカレントディレクトリを取得
        let dest_dir = match self.active_pane {
            PaneSide::Left => self.right_pane.current_tab().cwd.clone(),
            PaneSide::Right => self.left_pane.current_tab().cwd.clone(),
        };

        // 上書きの可能性をチェック
        let has_conflicts = paths.iter().any(|src| {
            if let Some(file_name) = src.file_name() {
                dest_dir.join(file_name).exists()
            } else {
                false
            }
        });

        // 上書き確認が必要な場合
        if has_conflicts && self.config.general.confirm_overwrite {
            let action = PendingAction::MoveToOtherPane { paths, dest_dir };
            self.confirm_message = action.message();
            self.pending_action = Some(action);
            self.mode = InputMode::Confirm;
            return;
        }

        // 確認なしで即実行
        self.execute_move_to_other_pane(paths, dest_dir);
    }

    fn execute_move_to_other_pane(&mut self, paths: Vec<PathBuf>, dest_dir: PathBuf) {
        let mut success_count = 0;
        let mut errors = Vec::new();

        for src in &paths {
            if let Some(file_name) = src.file_name() {
                let dest = dest_dir.join(file_name);
                match crate::fs::ops::move_file(src, &dest) {
                    Ok(_) => success_count += 1,
                    Err(e) => errors.push(format!("{}: {}", src.display(), e)),
                }
            }
        }

        // 両ペインをリロード
        self.left_pane.current_tab_mut().reload();
        self.right_pane.current_tab_mut().reload();

        // 選択をクリア
        self.active_pane_mut().current_tab_mut().clear_selection();

        // メッセージ表示
        if errors.is_empty() {
            self.show_message(&format!("Moved {} item(s) to other pane", success_count));
        } else {
            let error_msg = format!(
                "Moved {} item(s), {} error(s):\n{}",
                success_count,
                errors.len(),
                errors.join("\n")
            );
            self.show_error(&error_msg);
        }
    }

    fn delete_files(&mut self) {
        // 先に設定を読み取る
        let confirm_delete = self.config.general.confirm_delete;

        let tab = self.active_pane_mut().current_tab_mut();
        let paths = if tab.selection.is_empty() {
            if let Some(entry) = tab.current_entry() {
                vec![entry.path.clone()]
            } else {
                return;
            }
        } else {
            tab.get_selected_paths()
        };

        // 確認が必要な場合
        if confirm_delete {
            let action = PendingAction::Delete { paths };
            self.confirm_message = action.message();
            self.pending_action = Some(action);
            self.mode = InputMode::Confirm;
        } else {
            // 確認なしで即実行
            let mut errors = Vec::new();
            for path in &paths {
                if let Err(e) = crate::fs::ops::delete_file(path) {
                    errors.push(format!("{}: {}", path.display(), e));
                }
            }
            self.active_pane_mut().current_tab_mut().clear_selection();
            self.active_pane_mut().current_tab_mut().reload();

            if !errors.is_empty() {
                let error_msg = if errors.len() == 1 {
                    format!("Failed to delete file:\n{}", errors[0])
                } else {
                    format!("Failed to delete {} files:\n{}", errors.len(), errors.join("\n"))
                };
                self.show_error(&error_msg);
            }
        }
    }

    fn execute_command(&mut self, input: &str) {
        // コロンで始まるコマンドの処理
        if input.starts_with(':') {
            let cmd = input[1..].trim();
            match cmd {
                "config" => {
                    self.edit_config_file();
                }
                "keymap" => {
                    self.edit_keymap_file();
                }
                "theme" => {
                    self.edit_theme_file();
                }
                "opener" => {
                    self.edit_opener_file();
                }
                _ => {
                    self.status_message = format!("Unknown command: {}", cmd);
                }
            }
            self.command_prompt.clear();
            return;
        }

        if self.command_prompt.starts_with("Rename") {
            if !input.is_empty() {
                self.rename_file(input);
            }
        } else if self.command_prompt.starts_with("Create") {
            if !input.is_empty() {
                self.create_file(input);
            }
        } else if self.command_prompt.starts_with("Filter") {
            if input.is_empty() {
                self.active_pane_mut().current_tab_mut().set_filter(None);
            } else {
                self.active_pane_mut()
                    .current_tab_mut()
                    .set_filter(Some(input.to_string()));
            }
        } else if self.command_prompt.starts_with("Bookmark") {
            if !input.is_empty() {
                let path = self.active_pane().current_tab().cwd.clone();

                // "名前 キー" または "名前" をパース
                let parts: Vec<&str> = input.rsplitn(2, ' ').collect();
                if parts.len() == 2 && parts[0].len() == 1 {
                    // "名前 キー" 形式（最後がスペース+1文字）
                    let key_char = parts[0].chars().next().unwrap();
                    let name = parts[1].to_string();
                    self.bookmarks.add_with_key(name, path, key_char);
                    self.status_message = format!("Registered bookmark '{}' with key '{}'", parts[1], key_char);
                } else {
                    // "名前" のみ
                    self.bookmarks.add(input.to_string(), path);
                    self.status_message = format!("Added bookmark '{}'", input);
                }

                let _ = self.bookmarks.save();
            }
        }

        self.command_prompt.clear();
    }

    fn rename_file(&mut self, new_name: &str) {
        let tab = self.active_pane().current_tab();
        if let Some(entry) = tab.current_entry() {
            let old_path = entry.path.clone();
            let new_path = tab.cwd.join(new_name);
            let _ = std::fs::rename(old_path, new_path);
            self.active_pane_mut().current_tab_mut().reload();
        }
    }

    fn create_file(&mut self, name: &str) {
        let tab = self.active_pane().current_tab();
        let path = tab.cwd.join(name);

        if name.ends_with('/') {
            let _ = std::fs::create_dir_all(&path);
        } else {
            let _ = std::fs::File::create(&path);
        }

        self.active_pane_mut().current_tab_mut().reload();
    }

    #[cfg(feature = "archive")]
    fn extract_archive(&mut self) {
        use crate::fs::archive::{extract_archive, is_archive};

        let tab = self.active_pane().current_tab();
        if let Some(entry) = tab.current_entry() {
            if is_archive(&entry.path) {
                let archive_path = entry.path.clone();
                let dest_dir = tab.cwd.clone();

                // 確認が必要な場合
                if self.config.general.confirm_overwrite {
                    let action = PendingAction::ExtractArchive {
                        archive_path,
                        dest_dir,
                    };
                    self.confirm_message = action.message();
                    self.pending_action = Some(action);
                    self.mode = InputMode::Confirm;
                } else {
                    // 確認なしで即実行
                    match extract_archive(&archive_path, &dest_dir) {
                        Ok(_) => {
                            self.active_pane_mut().current_tab_mut().reload();
                            self.show_message(&format!("Extracted: {}", archive_path.display()));
                        }
                        Err(e) => {
                            self.show_error(&format!("Failed to extract archive:\n{}", e));
                        }
                    }
                }
            }
        }
    }

    #[cfg(not(feature = "archive"))]
    fn extract_archive(&mut self) {
        self.show_error("Archive support requires 'archive' feature");
    }

    #[cfg(feature = "archive")]
    fn compress_to_zip(&mut self) {
        use crate::fs::archive::compress_to_zip;

        let tab = self.active_pane_mut().current_tab_mut();
        let files = if tab.selection.is_empty() {
            if let Some(entry) = tab.current_entry() {
                vec![entry.path.clone()]
            } else {
                return;
            }
        } else {
            tab.get_selected_paths()
        };

        if files.is_empty() {
            return;
        }

        let output_path = tab.cwd.join("archive.zip");

        match compress_to_zip(&files, &output_path) {
            Ok(_) => {
                tab.clear_selection();
                tab.reload();
                self.show_message(&format!("Compressed to: {}", output_path.display()));
            }
            Err(e) => {
                self.show_error(&format!("Failed to compress files:\n{}", e));
            }
        }
    }

    #[cfg(not(feature = "archive"))]
    fn compress_to_zip(&mut self) {
        self.show_error("Archive support requires 'archive' feature");
    }

    fn open_current_file(&mut self) {
        if let Some(entry) = self.active_pane().current_tab().current_entry() {
            if !entry.is_dir {
                let path = entry.path.clone();
                let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("file");
                match crate::fs::opener::open_file(&path, &self.config.opener) {
                    Ok(_) => {
                        self.show_message(&format!("Opened: {}", filename));
                    }
                    Err(e) => {
                        self.show_error(&format!("Failed to open file:\n{}", e));
                    }
                }
            } else {
                self.show_error("Cannot open directory");
            }
        }
    }

    fn open_with_editor(&mut self) {
        if let Some(entry) = self.active_pane().current_tab().current_entry() {
            if !entry.is_dir {
                let path = entry.path.clone();
                let editor = self.config.opener.editor.clone();
                // TUIを一時停止してエディタを起動
                self.suspend_for_command = Some((editor, path));
            } else {
                self.show_error("Cannot open directory with editor");
            }
        }
    }

    fn open_with_pager(&mut self) {
        if let Some(entry) = self.active_pane().current_tab().current_entry() {
            if !entry.is_dir {
                let path = entry.path.clone();
                let pager = self.config.opener.pager.clone();
                // TUIを一時停止してページャを起動
                self.suspend_for_command = Some((pager, path));
            } else {
                self.show_error("Cannot open directory with pager");
            }
        }
    }

    fn execute_current_file(&mut self) {
        if let Some(entry) = self.active_pane().current_tab().current_entry() {
            if entry.is_dir {
                self.show_error("Cannot execute directory");
            } else if !entry.is_executable {
                self.show_error("File is not executable");
            } else {
                let path = entry.path.clone();
                let path_str = path.to_string_lossy().to_string();
                // TUIを一時停止して実行可能ファイルを起動
                self.suspend_for_command = Some((path_str, path));
            }
        }
    }

    fn edit_config_file(&mut self) {
        use std::fs;
        let config_path = crate::config::get_config_path();

        // 設定ファイルが存在しない場合はデフォルトから作成
        if !config_path.exists() {
            if let Some(parent) = config_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let default_config = include_str!("../../config/default/echofiler.toml");
            if let Err(e) = fs::write(&config_path, default_config) {
                self.show_error(&format!("Failed to create config file:\n{}", e));
                return;
            }
        }

        let editor = self.config.opener.editor.clone();
        // TUIを一時停止してエディタを起動
        self.suspend_for_command = Some((editor, config_path));
    }

    fn edit_keymap_file(&mut self) {
        use std::fs;
        let keymap_path = crate::config::get_keymap_path();

        // 設定ファイルが存在しない場合はデフォルトから作成
        if !keymap_path.exists() {
            if let Some(parent) = keymap_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let default_keymap = include_str!("../../config/default/keymap.toml");
            if let Err(e) = fs::write(&keymap_path, default_keymap) {
                self.show_error(&format!("Failed to create keymap file:\n{}", e));
                return;
            }
        }

        let editor = self.config.opener.editor.clone();
        // TUIを一時停止してエディタを起動
        self.suspend_for_command = Some((editor, keymap_path));
    }

    fn edit_theme_file(&mut self) {
        use std::fs;
        let theme_path = crate::config::get_theme_path();

        // 設定ファイルが存在しない場合はデフォルトから作成
        if !theme_path.exists() {
            if let Some(parent) = theme_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let default_theme = include_str!("../../config/default/theme.toml");
            if let Err(e) = fs::write(&theme_path, default_theme) {
                self.show_error(&format!("Failed to create theme file:\n{}", e));
                return;
            }
        }

        let editor = self.config.opener.editor.clone();
        // TUIを一時停止してエディタを起動
        self.suspend_for_command = Some((editor, theme_path));
    }

    fn edit_opener_file(&mut self) {
        use std::fs;
        let opener_path = crate::config::get_opener_path();

        // 設定ファイルが存在しない場合はデフォルトから作成
        if !opener_path.exists() {
            if let Some(parent) = opener_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let default_opener = include_str!("../../config/default/opener.toml");
            if let Err(e) = fs::write(&opener_path, default_opener) {
                self.show_error(&format!("Failed to create opener file:\n{}", e));
                return;
            }
        }

        let editor = self.config.opener.editor.clone();
        // TUIを一時停止してエディタを起動
        self.suspend_for_command = Some((editor, opener_path));
    }

    fn complete_command(&mut self) {
        // 利用可能なコマンド一覧
        const COMMANDS: &[&str] = &["config", "keymap", "theme", "opener"];

        // 入力から`:`を除いた部分を取得
        let input = &self.command_input[1..];

        // 候補を絞り込み
        let matches: Vec<&str> = COMMANDS
            .iter()
            .filter(|cmd| cmd.starts_with(input))
            .copied()
            .collect();

        match matches.len() {
            0 => {
                // 候補なし
                self.status_message = "No matching commands".to_string();
            }
            1 => {
                // 候補が1つ: 自動補完
                self.command_input = format!(":{}", matches[0]);
                self.status_message.clear();
            }
            _ => {
                // 候補が複数: 候補を表示
                self.status_message = format!("Suggestions: {}", matches.join(", "));
            }
        }
    }

    /// メッセージダイアログを表示
    pub fn show_message(&mut self, message: &str) {
        self.dialog_message = message.to_string();
        self.is_error_dialog = false;
        self.mode = InputMode::MessageDialog;
    }

    /// エラーダイアログを表示
    pub fn show_error(&mut self, message: &str) {
        self.dialog_message = message.to_string();
        self.is_error_dialog = true;
        self.mode = InputMode::MessageDialog;
    }
}
