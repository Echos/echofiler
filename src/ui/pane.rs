use crate::core::Pane;
use crate::config::general::IconStyle;
use crate::config::theme::ThemeConfig;
use crate::ui::icons;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, List, ListItem, ListState, StatefulWidget, Widget},
};

pub struct PaneWidget<'a> {
    pane: &'a Pane,
    is_active: bool,
    theme: &'a ThemeConfig,
    show_icons: bool,
    icon_style: IconStyle,
    icon_spacing: u8,
}

impl<'a> PaneWidget<'a> {
    pub fn new(
        pane: &'a Pane,
        is_active: bool,
        theme: &'a ThemeConfig,
        show_icons: bool,
        icon_style: IconStyle,
        icon_spacing: u8,
    ) -> Self {
        Self {
            pane,
            is_active,
            theme,
            show_icons,
            icon_style,
            icon_spacing,
        }
    }
}

impl<'a> PaneWidget<'a> {
    /// render関数の実装（スクロール対応）
    pub fn render_with_scroll(self, area: Rect, buf: &mut Buffer) {
        let tab = self.pane.current_tab();
        let border_style = if self.is_active {
            self.theme.ui.border_focused.to_style()
        } else {
            self.theme.ui.border.to_style()
        };

        let tab_info = if self.pane.tabs.len() > 1 {
            format!(" [{}/{}] {} ", self.pane.active_tab + 1, self.pane.tabs.len(), tab.cwd.display())
        } else {
            format!(" {} ", tab.cwd.display())
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(tab_info);

        let items: Vec<ListItem> = tab
            .entries
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let selected = tab.is_selected(i);
                let text = if self.show_icons {
                    let icon = icons::get_icon(&entry.path, entry.is_dir, entry.is_symlink, self.icon_style);
                    let spacing = " ".repeat(self.icon_spacing as usize);
                    let prefix = if selected {
                        "● "
                    } else {
                        ""
                    };
                    format!("{}{}{}{}", prefix, icon, spacing, entry.name)
                } else {
                    let prefix = if selected {
                        "● "
                    } else if entry.is_dir {
                        "▶ "
                    } else {
                        "  "
                    };
                    format!("{}{}", prefix, entry.name)
                };
                let style = if i == tab.cursor {
                    self.theme.ui.cursor.to_style()
                } else if selected {
                    self.theme.ui.selection.to_style()
                } else if entry.is_dir {
                    self.theme.file.directory.to_style()
                } else if entry.name.starts_with('.') {
                    self.theme.file.hidden.to_style()
                } else if entry.is_executable {
                    self.theme.file.executable.to_style()
                } else if entry.is_symlink {
                    self.theme.file.symlink.to_style()
                } else {
                    Style::default()
                };
                ListItem::new(text).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(self.theme.ui.cursor.to_style());

        // ListStateを作成してスクロールオフセットとカーソル位置を設定
        let mut state = ListState::default();
        state.select(Some(tab.cursor));

        // 手動でスクロールオフセットを設定（自動スクロールを無効化）
        *state.offset_mut() = tab.scroll_offset;

        // StatefulWidgetとして描画（スクロールオフセットを反映）
        StatefulWidget::render(list, area, buf, &mut state);
    }
}

impl<'a> Widget for PaneWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_with_scroll(area, buf);
    }
}
