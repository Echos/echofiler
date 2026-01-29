use crate::app::App;
use crate::core::ClipboardMode;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Paragraph, Widget},
};

pub struct Statusline<'a> {
    app: &'a App,
}

impl<'a> Statusline<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

impl<'a> Widget for Statusline<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let tab = self.app.active_pane().current_tab();
        let mode_text = format!("[{:?}]", self.app.mode);

        let selected_count = tab.selection.len();
        let count_text = if selected_count > 0 {
            format!("{} selected | {} items", selected_count, tab.entries.len())
        } else {
            format!("{} items", tab.entries.len())
        };

        let clipboard_text = if !self.app.clipboard.is_empty() {
            match &self.app.clipboard.mode {
                Some(ClipboardMode::Copy) => {
                    format!(" | Clipboard: {} copied", self.app.clipboard.paths.len())
                }
                Some(ClipboardMode::Cut) => {
                    format!(" | Clipboard: {} cut", self.app.clipboard.paths.len())
                }
                None => String::new(),
            }
        } else {
            String::new()
        };

        let path_text = format!("{}", tab.cwd.display());

        // ステータスラインのスタイル（背景を暗く、文字を明るく）
        let style = Style::default().bg(Color::Black).fg(Color::White);

        // status_messageがある場合は、それを優先的に表示
        if !self.app.status_message.is_empty() {
            let message_style = if self.app.status_message.starts_with("Error:") {
                Style::default().bg(Color::Black).fg(Color::LightRed)
            } else {
                Style::default().bg(Color::Black).fg(Color::LightGreen)
            };

            let spans = vec![
                Span::styled(format!("{} | {} | ", mode_text, count_text), style),
                Span::styled(&self.app.status_message, message_style),
                Span::styled(format!(" | {}{}", path_text, clipboard_text), style),
            ];

            let paragraph = Paragraph::new(ratatui::text::Line::from(spans));
            Widget::render(paragraph, area, buf);
        } else {
            let text = format!(
                "{} | {} | {}{}",
                mode_text, count_text, path_text, clipboard_text
            );

            let paragraph = Paragraph::new(Span::styled(text, style));
            Widget::render(paragraph, area, buf);
        }
    }
}
