use crate::config::theme::ThemeConfig;
use crate::core::BookmarkList;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Borders, List, ListItem, Widget},
};

pub struct BookmarkWidget<'a> {
    bookmarks: &'a BookmarkList,
    cursor: usize,
    theme: &'a ThemeConfig,
}

impl<'a> BookmarkWidget<'a> {
    pub fn new(bookmarks: &'a BookmarkList, cursor: usize, theme: &'a ThemeConfig) -> Self {
        Self {
            bookmarks,
            cursor,
            theme,
        }
    }
}

impl<'a> Widget for BookmarkWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_style = self.theme.ui.border_focused.to_style();

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(" Bookmarks ");

        let items: Vec<ListItem> = if self.bookmarks.bookmarks.is_empty() {
            vec![ListItem::new("No bookmarks. Press 'b' to add.")]
        } else {
            self.bookmarks
                .bookmarks
                .iter()
                .enumerate()
                .map(|(i, bookmark)| {
                    let key_str = if let Some(key) = bookmark.key {
                        format!("[{}] ", key)
                    } else {
                        "    ".to_string()
                    };
                    let text = format!("{}{}: {}", key_str, bookmark.name, bookmark.path.display());
                    let style = if i == self.cursor {
                        self.theme.ui.cursor.to_style()
                    } else {
                        Style::default()
                    };
                    ListItem::new(text).style(style)
                })
                .collect()
        };

        let help_text = if !self.bookmarks.bookmarks.is_empty() {
            "j/k: move | Enter/key: jump | d: delete | q/Esc: close"
        } else {
            "q/Esc: close"
        };

        let items_with_help = {
            let mut all_items = items;
            all_items.push(ListItem::new(""));
            all_items.push(ListItem::new(Line::from(help_text)));
            all_items
        };

        let list = List::new(items_with_help).block(block);
        Widget::render(list, area, buf);
    }
}
