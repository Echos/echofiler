use crate::config::theme::ThemeConfig;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct HelpWidget<'a> {
    theme: &'a ThemeConfig,
}

impl<'a> HelpWidget<'a> {
    pub fn new(theme: &'a ThemeConfig) -> Self {
        Self { theme }
    }

    fn build_help_lines(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        lines.push(Line::from(Span::styled(
            "=== echofiler - Keyboard Shortcuts ===",
            Style::default().add_modifier(ratatui::style::Modifier::BOLD),
        )));
        lines.push(Line::from(""));

        // ナビゲーション
        lines.push(Line::from(Span::styled(
            "[ Navigation ]",
            Style::default().add_modifier(ratatui::style::Modifier::BOLD),
        )));
        lines.push(Line::from("  j / ↓         Move cursor down"));
        lines.push(Line::from("  k / ↑         Move cursor up"));
        lines.push(Line::from("  Enter         Enter directory / Preview file"));
        lines.push(Line::from("  Backspace     Go to parent directory"));
        lines.push(Line::from(""));

        // ペイン
        lines.push(Line::from(Span::styled(
            "[ Pane ]",
            Style::default().add_modifier(ratatui::style::Modifier::BOLD),
        )));
        lines.push(Line::from("  Tab           Toggle active pane"));
        lines.push(Line::from("  ← / Ctrl+h    Focus left pane"));
        lines.push(Line::from("  → / Ctrl+l    Focus right pane"));
        lines.push(Line::from(""));

        // ファイル選択
        lines.push(Line::from(Span::styled(
            "[ File Selection ]",
            Style::default().add_modifier(ratatui::style::Modifier::BOLD),
        )));
        lines.push(Line::from("  Space         Toggle file selection"));
        lines.push(Line::from("  v             Visual mode (continuous selection)"));
        lines.push(Line::from("  Esc           Return to Normal mode"));
        lines.push(Line::from(""));

        // ファイル操作
        lines.push(Line::from(Span::styled(
            "[ File Operations ]",
            Style::default().add_modifier(ratatui::style::Modifier::BOLD),
        )));
        lines.push(Line::from("  y             Yank (copy) selected files"));
        lines.push(Line::from("  d             Cut selected files"));
        lines.push(Line::from("  p             Paste files to opposite pane"));
        lines.push(Line::from("  D             Delete selected files"));
        lines.push(Line::from("  C             Copy to other pane directly"));
        lines.push(Line::from("  M             Move to other pane directly"));
        lines.push(Line::from("  R             Rename file"));
        lines.push(Line::from("  a             Create new file/directory"));
        lines.push(Line::from("  e             Extract archive (with archive feature)"));
        lines.push(Line::from("  z             Compress to ZIP (with archive feature)"));
        lines.push(Line::from(""));

        // ファイルを開く
        lines.push(Line::from(Span::styled(
            "[ Open Files ]",
            Style::default().add_modifier(ratatui::style::Modifier::BOLD),
        )));
        lines.push(Line::from("  o             Open file with default application"));
        lines.push(Line::from("  E             Open file with editor ($EDITOR)"));
        lines.push(Line::from("  w             Open file with pager ($PAGER)"));
        lines.push(Line::from("  X             Execute file"));
        lines.push(Line::from(""));

        // 検索・フィルター
        lines.push(Line::from(Span::styled(
            "[ Search & Filter ]",
            Style::default().add_modifier(ratatui::style::Modifier::BOLD),
        )));
        lines.push(Line::from("  /             Search mode"));
        lines.push(Line::from("  n             Next search result"));
        lines.push(Line::from("  N             Previous search result"));
        lines.push(Line::from("  f             Filter by filename"));
        lines.push(Line::from(""));

        // タブ
        lines.push(Line::from(Span::styled(
            "[ Tabs ]",
            Style::default().add_modifier(ratatui::style::Modifier::BOLD),
        )));
        lines.push(Line::from("  Ctrl+t        New tab"));
        lines.push(Line::from("  Ctrl+w        Close tab"));
        lines.push(Line::from("  Ctrl+l        Clear screen and refresh"));
        lines.push(Line::from("  h             Previous tab"));
        lines.push(Line::from("  l             Next tab"));
        lines.push(Line::from("  [ / ]         Previous / Next tab (alternative)"));
        lines.push(Line::from(""));

        // ビュー
        lines.push(Line::from(Span::styled(
            "[ View ]",
            Style::default().add_modifier(ratatui::style::Modifier::BOLD),
        )));
        lines.push(Line::from("  P             Toggle preview mode"));
        lines.push(Line::from("  i             Toggle sidebar"));
        lines.push(Line::from("  .             Toggle hidden files"));
        lines.push(Line::from("  s             Cycle sort method"));
        lines.push(Line::from("  S             Toggle sort reverse"));
        lines.push(Line::from(""));

        // ブックマーク
        lines.push(Line::from(Span::styled(
            "[ Bookmarks (Prefix: g) ]",
            Style::default().add_modifier(ratatui::style::Modifier::BOLD),
        )));
        lines.push(Line::from("  g             Enter bookmark prefix mode"));
        lines.push(Line::from("  g b           Add bookmark (name [key])"));
        lines.push(Line::from("  g B           Show all bookmarks"));
        lines.push(Line::from("  g m           Show bookmarks with keys"));
        lines.push(Line::from("  g + key       Jump to bookmark by key"));
        lines.push(Line::from(""));

        // 設定編集
        lines.push(Line::from(Span::styled(
            "[ Configuration ]",
            Style::default().add_modifier(ratatui::style::Modifier::BOLD),
        )));
        lines.push(Line::from("  :config       Edit echofiler.toml"));
        lines.push(Line::from("  :keymap       Edit keymap.toml"));
        lines.push(Line::from("  :theme        Edit theme.toml"));
        lines.push(Line::from("  :opener       Edit opener.toml"));
        lines.push(Line::from("  Tab           Complete command"));
        lines.push(Line::from(""));

        // その他
        lines.push(Line::from(Span::styled(
            "[ Other ]",
            Style::default().add_modifier(ratatui::style::Modifier::BOLD),
        )));
        lines.push(Line::from("  r             Refresh view"));
        lines.push(Line::from("  ?             Show this help"));
        lines.push(Line::from("  q             Quit / Close help"));
        lines.push(Line::from(""));

        lines.push(Line::from("Press q or Esc to close this help"));

        lines
    }
}

impl<'a> Widget for HelpWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // 中央にポップアップ風のダイアログを表示
        let border_style = self.theme.ui.border_focused.to_style();

        // 画面の中央に表示するために、サイズを計算
        let popup_width = area.width.min(80);
        let popup_height = area.height.min(40);
        let popup_x = (area.width.saturating_sub(popup_width)) / 2;
        let popup_y = (area.height.saturating_sub(popup_height)) / 2;

        let popup_area = Rect {
            x: area.x + popup_x,
            y: area.y + popup_y,
            width: popup_width,
            height: popup_height,
        };

        // 背景をクリア（疑似的な半透明効果）
        for y in popup_area.y..popup_area.y + popup_area.height {
            for x in popup_area.x..popup_area.x + popup_area.width {
                if x < area.width && y < area.height {
                    buf[(x, y)].set_char(' ');
                }
            }
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(" Help - Keyboard Shortcuts ");

        let lines = self.build_help_lines();
        let paragraph = Paragraph::new(lines).block(block);

        Widget::render(paragraph, popup_area, buf);
    }
}
