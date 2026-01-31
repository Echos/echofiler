use crate::config::theme::ThemeConfig;
use crate::ui::image_preview::{is_image_file, render_image_preview};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};
use std::fs;
use std::path::Path;

#[cfg(feature = "archive")]
use crate::fs::archive::{is_archive, list_archive_contents};

pub struct PreviewWidget<'a> {
    path: Option<&'a Path>,
    theme: &'a ThemeConfig,
    scroll: usize,
}

impl<'a> PreviewWidget<'a> {
    pub fn new(path: Option<&'a Path>, theme: &'a ThemeConfig, scroll: usize) -> Self {
        Self { path, theme, scroll }
    }

    fn read_preview(&self, area: Rect) -> Vec<Line<'static>> {
        let Some(path) = self.path else {
            return vec![Line::from("No file selected")];
        };

        if !path.exists() {
            return vec![Line::from("File not found")];
        }

        if path.is_dir() {
            return vec![Line::from("Directory preview not implemented")];
        }

        // 画像ファイルの場合
        if is_image_file(path) {
            // ボーダー分を考慮してサイズを調整
            let max_width = area.width.saturating_sub(2);
            let max_height = area.height.saturating_sub(2);
            return render_image_preview(path, max_width, max_height);
        }

        // アーカイブファイルの場合
        #[cfg(feature = "archive")]
        if is_archive(path) {
            return match list_archive_contents(path) {
                Ok(contents) => {
                    let mut lines = vec![Line::from("Archive contents:")];
                    lines.push(Line::from(""));
                    for content in contents.iter().take(100) {
                        lines.push(Line::from(content.to_string()));
                    }
                    if contents.len() > 100 {
                        lines.push(Line::from(""));
                        lines.push(Line::from(format!(
                            "... and {} more files",
                            contents.len() - 100
                        )));
                    }
                    lines
                }
                Err(e) => vec![Line::from(format!("Failed to read archive: {}", e))],
            };
        }

        // ファイルサイズ制限（10MB）
        if let Ok(metadata) = fs::metadata(path) {
            if metadata.len() > 10 * 1024 * 1024 {
                return vec![Line::from("File too large for preview")];
            }
        }

        // テキストファイルを読み込み
        match fs::read_to_string(path) {
            Ok(content) => {
                content
                    .lines()
                    .take(100) // 最初の100行のみ
                    .map(|line| Line::from(line.to_string()))
                    .collect()
            }
            Err(_) => vec![Line::from("Cannot preview binary file")],
        }
    }
}

impl<'a> Widget for PreviewWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_style = self.theme.ui.border.to_style();

        let title = if let Some(path) = self.path {
            format!(" Preview: {} ", path.display())
        } else {
            " Preview ".to_string()
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(title);

        let lines = self.read_preview(area);

        // スクロールを適用（最大値を制限）
        let max_scroll = lines.len().saturating_sub(area.height.saturating_sub(2) as usize);
        let scroll = self.scroll.min(max_scroll);

        let paragraph = Paragraph::new(lines)
            .block(block)
            .scroll((scroll as u16, 0));

        Widget::render(paragraph, area, buf);
    }
}
