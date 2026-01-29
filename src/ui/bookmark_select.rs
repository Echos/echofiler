use crate::core::BookmarkList;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct BookmarkSelectDialog<'a> {
    pub bookmarks: &'a BookmarkList,
}

impl<'a> BookmarkSelectDialog<'a> {
    pub fn new(bookmarks: &'a BookmarkList) -> Self {
        Self { bookmarks }
    }
}

impl<'a> Widget for BookmarkSelectDialog<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // ダイアログサイズを計算
        let dialog_width = 60.min(area.width.saturating_sub(4));
        let dialog_height = (self.bookmarks.bookmarks.len() as u16 + 4).min(area.height.saturating_sub(4));

        // 中央に配置
        let dialog_area = Rect {
            x: area.x + (area.width.saturating_sub(dialog_width)) / 2,
            y: area.y + (area.height.saturating_sub(dialog_height)) / 2,
            width: dialog_width,
            height: dialog_height,
        };

        // 背景をクリア
        for y in dialog_area.y..dialog_area.y + dialog_area.height {
            for x in dialog_area.x..dialog_area.x + dialog_area.width {
                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.set_char(' ');
                    cell.set_style(Style::default().bg(Color::Black));
                }
            }
        }

        // ブックマーク一覧を作成
        let mut lines = vec![
            Line::from(vec![Span::styled(
                "Bookmarks (press key to jump, Esc to cancel)",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
        ];

        if self.bookmarks.bookmarks.is_empty() {
            lines.push(Line::from(vec![Span::styled(
                "No bookmarks registered. Press 'm' + key to register.",
                Style::default().fg(Color::Yellow),
            )]));
        } else {
            for bookmark in &self.bookmarks.bookmarks {
                let key_str = if let Some(key) = bookmark.key {
                    format!("  {}: ", key)
                } else {
                    "  -: ".to_string()
                };

                lines.push(Line::from(vec![
                    Span::styled(key_str, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                    Span::styled(&bookmark.name, Style::default().fg(Color::White)),
                    Span::styled(" → ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        bookmark.path.display().to_string(),
                        Style::default().fg(Color::Blue),
                    ),
                ]));
            }
        }

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan))
                    .title(" Bookmarks "),
            )
            .alignment(Alignment::Left);

        paragraph.render(dialog_area, buf);
    }
}
