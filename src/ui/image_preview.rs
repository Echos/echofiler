use ratatui::text::Line;
use std::path::Path;

#[cfg(feature = "preview")]
use image::{DynamicImage, GenericImageView};
#[cfg(feature = "preview")]
use ratatui::style::{Color, Style};
#[cfg(feature = "preview")]
use ratatui::text::Span;

/// 画像ファイルかどうかを拡張子から判定
pub fn is_image_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        matches!(
            ext.to_str().unwrap_or("").to_lowercase().as_str(),
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "ico" | "tiff" | "tif"
        )
    } else {
        false
    }
}

#[cfg(feature = "preview")]
pub fn render_image_preview(path: &Path, max_width: u16, max_height: u16) -> Vec<Line<'static>> {
    match load_and_resize_image(path, max_width, max_height) {
        Ok(lines) => lines,
        Err(e) => vec![Line::from(format!("Failed to load image: {}", e))],
    }
}

#[cfg(not(feature = "preview"))]
pub fn render_image_preview(_path: &Path, _max_width: u16, _max_height: u16) -> Vec<Line<'static>> {
    vec![Line::from("Image preview requires 'preview' feature")]
}

#[cfg(feature = "preview")]
fn load_and_resize_image(
    path: &Path,
    max_width: u16,
    max_height: u16,
) -> Result<Vec<Line<'static>>, String> {
    // 画像を読み込み
    let img = image::open(path).map_err(|e| e.to_string())?;

    // リサイズ計算（半分の高さ、Unicodeブロック文字で2ピクセルを1文字で表現）
    let (img_width, img_height) = img.dimensions();
    let target_height = (max_height as u32).min(img_height / 2);
    let target_width = (max_width as u32).min(img_width);

    // アスペクト比を保持してリサイズ
    let scale_w = target_width as f32 / img_width as f32;
    let scale_h = target_height as f32 / (img_height as f32 / 2.0);
    let scale = scale_w.min(scale_h);

    let new_width = (img_width as f32 * scale) as u32;
    let new_height = (img_height as f32 * scale) as u32;

    let resized = img.resize_exact(new_width, new_height, image::imageops::FilterType::Triangle);

    // Unicodeブロック文字でレンダリング
    Ok(render_with_blocks(&resized))
}

#[cfg(feature = "preview")]
fn render_with_blocks(img: &DynamicImage) -> Vec<Line<'static>> {
    let (width, height) = img.dimensions();
    let mut lines = Vec::new();

    // 2行のピクセルを1つのブロック文字で表現
    for y in (0..height).step_by(2) {
        let mut spans = Vec::new();
        for x in 0..width {
            let top_pixel = img.get_pixel(x, y);
            let bottom_pixel = if y + 1 < height {
                img.get_pixel(x, y + 1)
            } else {
                top_pixel
            };

            let top_rgb = [top_pixel[0], top_pixel[1], top_pixel[2]];
            let bottom_rgb = [bottom_pixel[0], bottom_pixel[1], bottom_pixel[2]];

            // 上半分と下半分の色を使ってブロック文字を選択
            let (ch, fg, bg) = select_block_char(&top_rgb, &bottom_rgb);

            spans.push(Span::styled(
                ch.to_string(),
                Style::default().fg(fg).bg(bg),
            ));
        }
        lines.push(Line::from(spans));
    }

    lines
}

#[cfg(feature = "preview")]
fn select_block_char(top: &[u8; 3], bottom: &[u8; 3]) -> (char, Color, Color) {
    let top_color = rgb_to_color(top);
    let bottom_color = rgb_to_color(bottom);

    // 上半分ブロック（▀）を使用: 前景=上の色、背景=下の色
    ('▀', top_color, bottom_color)
}

#[cfg(feature = "preview")]
fn rgb_to_color(rgb: &[u8; 3]) -> Color {
    Color::Rgb(rgb[0], rgb[1], rgb[2])
}
