use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Paragraph, Widget},
};

pub struct CommandLine<'a> {
    pub prompt: &'a str,
    pub input: &'a str,
}

impl<'a> CommandLine<'a> {
    pub fn new(prompt: &'a str, input: &'a str) -> Self {
        Self { prompt, input }
    }
}

impl<'a> Widget for CommandLine<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = if self.prompt.is_empty() {
            self.input.to_string()
        } else {
            format!("{}{}", self.prompt, self.input)
        };

        let paragraph = Paragraph::new(Span::styled(
            text,
            Style::default().bg(Color::Black).fg(Color::White),
        ));

        Widget::render(paragraph, area, buf);
    }
}
