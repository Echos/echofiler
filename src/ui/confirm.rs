use crate::config::theme::ThemeConfig;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

pub struct ConfirmWidget<'a> {
    message: &'a str,
    theme: &'a ThemeConfig,
}

impl<'a> ConfirmWidget<'a> {
    pub fn new(message: &'a str, theme: &'a ThemeConfig) -> Self {
        Self { message, theme }
    }
}

impl<'a> Widget for ConfirmWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // ダイアログサイズ
        let dialog_width = area.width.min(60);
        let dialog_height = 7;

        // 中央配置
        let dialog_x = (area.width.saturating_sub(dialog_width)) / 2;
        let dialog_y = (area.height.saturating_sub(dialog_height)) / 2;

        let dialog_area = Rect {
            x: area.x + dialog_x,
            y: area.y + dialog_y,
            width: dialog_width,
            height: dialog_height,
        };

        // 背景をクリア
        for y in dialog_area.y..dialog_area.y + dialog_area.height {
            for x in dialog_area.x..dialog_area.x + dialog_area.width {
                if x < area.width && y < area.height {
                    buf[(x, y)].set_char(' ');
                }
            }
        }

        // ボーダースタイル
        let border_style = self.theme.ui.border.to_style();

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(" Confirm ");

        // メッセージと操作案内を作成
        let lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                self.message,
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::raw("")),
            Line::from(vec![
                Span::styled("y", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": Yes  "),
                Span::styled("n", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": No  "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": Cancel"),
            ]),
        ];

        let paragraph = Paragraph::new(lines)
            .block(block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        Widget::render(paragraph, dialog_area, buf);
    }
}
