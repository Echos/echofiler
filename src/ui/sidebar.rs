use crate::config::theme::ThemeConfig;
use crate::core::Entry;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct SidebarWidget<'a> {
    entry: Option<&'a Entry>,
    theme: &'a ThemeConfig,
}

impl<'a> SidebarWidget<'a> {
    pub fn new(entry: Option<&'a Entry>, theme: &'a ThemeConfig) -> Self {
        Self { entry, theme }
    }

    fn format_size(size: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = 1024 * KB;
        const GB: u64 = 1024 * MB;

        if size >= GB {
            format!("{:.2} GB", size as f64 / GB as f64)
        } else if size >= MB {
            format!("{:.2} MB", size as f64 / MB as f64)
        } else if size >= KB {
            format!("{:.2} KB", size as f64 / KB as f64)
        } else {
            format!("{} B", size)
        }
    }

    fn format_time(time: &Option<std::time::SystemTime>) -> String {
        use chrono::{DateTime, Local};

        match time {
            Some(t) => {
                let datetime: DateTime<Local> = (*t).into();
                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
            }
            None => "Unknown".to_string(),
        }
    }

    #[cfg(unix)]
    fn format_permissions(entry: &Entry) -> String {
        use std::os::unix::fs::PermissionsExt;

        if let Ok(metadata) = std::fs::metadata(&entry.path) {
            let mode = metadata.permissions().mode();
            let user = (mode >> 6) & 0o7;
            let group = (mode >> 3) & 0o7;
            let other = mode & 0o7;
            format!("{:o}{:o}{:o}", user, group, other)
        } else {
            "---".to_string()
        }
    }

    #[cfg(not(unix))]
    fn format_permissions(_entry: &Entry) -> String {
        "N/A".to_string()
    }

    fn build_info_lines(&self) -> Vec<Line<'static>> {
        let Some(entry) = self.entry else {
            return vec![Line::from("No file selected")];
        };

        let mut lines = Vec::new();

        // ファイル名
        lines.push(Line::from(vec![
            Span::raw("Name: "),
            Span::styled(entry.name.clone(), self.theme.file.directory.to_style()),
        ]));
        lines.push(Line::from(""));

        // タイプ
        let file_type = if entry.is_dir {
            "Directory"
        } else if entry.is_symlink {
            "Symlink"
        } else {
            "File"
        };
        lines.push(Line::from(format!("Type: {}", file_type)));

        // サイズ
        if !entry.is_dir {
            lines.push(Line::from(format!("Size: {}", Self::format_size(entry.size))));
        }

        // パーミッション
        lines.push(Line::from(format!("Permissions: {}", Self::format_permissions(entry))));

        // 変更日時
        lines.push(Line::from(format!("Modified: {}", Self::format_time(&entry.modified))));

        // パス
        lines.push(Line::from(""));
        lines.push(Line::from("Path:"));
        lines.push(Line::from(format!("{}", entry.path.display())));

        lines
    }
}

impl<'a> Widget for SidebarWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_style = self.theme.ui.border.to_style();

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(" Info ");

        let lines = self.build_info_lines();
        let paragraph = Paragraph::new(lines).block(block);

        Widget::render(paragraph, area, buf);
    }
}
