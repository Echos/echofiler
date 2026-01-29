use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

pub struct MessageDialog<'a> {
    message: &'a str,
    is_error: bool,
}

impl<'a> MessageDialog<'a> {
    pub fn new(message: &'a str, is_error: bool) -> Self {
        Self { message, is_error }
    }
}

impl<'a> Widget for MessageDialog<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // ダイアログの背景を暗くする
        let block = Block::default()
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::Black));
        Widget::render(block, area, buf);

        // ダイアログのサイズを計算
        let dialog_width = area.width.min(80);
        let dialog_height = 10;
        let dialog_area = Rect {
            x: (area.width.saturating_sub(dialog_width)) / 2,
            y: (area.height.saturating_sub(dialog_height)) / 2,
            width: dialog_width,
            height: dialog_height,
        };

        // タイトルとボーダーの色を設定
        let (title, border_color, title_color) = if self.is_error {
            ("Error", Color::Red, Color::LightRed)
        } else {
            ("Message", Color::Green, Color::LightGreen)
        };

        // ダイアログボックス
        let dialog_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(Span::styled(
                format!(" {} ", title),
                Style::default()
                    .fg(title_color)
                    .add_modifier(Modifier::BOLD),
            ))
            .style(Style::default().bg(Color::Black).fg(Color::White));

        let inner_area = dialog_block.inner(dialog_area);
        Widget::render(dialog_block, dialog_area, buf);

        // メッセージテキストを分割
        let text_lines: Vec<Line> = self
            .message
            .lines()
            .map(|line| Line::from(line.to_string()))
            .collect();

        // 下部のヘルプテキスト用にスペースを確保
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(inner_area);

        // メッセージテキスト
        let message_paragraph = Paragraph::new(text_lines)
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White));
        Widget::render(message_paragraph, chunks[0], buf);

        // ヘルプテキスト
        let help_text = Line::from(vec![Span::styled(
            "Press any key to close",
            Style::default().fg(Color::DarkGray),
        )]);
        let help_paragraph = Paragraph::new(help_text).alignment(Alignment::Center);
        Widget::render(help_paragraph, chunks[1], buf);
    }
}
