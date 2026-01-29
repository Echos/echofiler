use serde::Deserialize;
use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone, Deserialize)]
pub struct ThemeConfig {
    #[serde(default)]
    pub colors: ColorConfig,
    #[serde(default)]
    pub file: FileColorConfig,
    #[serde(default)]
    pub ui: UiColorConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ColorConfig {
    #[serde(default = "default_background")]
    pub background: String,
    #[serde(default = "default_foreground")]
    pub foreground: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FileColorConfig {
    #[serde(default)]
    pub directory: StyleConfig,
    #[serde(default)]
    pub executable: StyleConfig,
    #[serde(default)]
    pub symlink: StyleConfig,
    #[serde(default)]
    pub hidden: StyleConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UiColorConfig {
    #[serde(default)]
    pub border: StyleConfig,
    #[serde(default)]
    pub border_focused: StyleConfig,
    #[serde(default)]
    pub cursor: StyleConfig,
    #[serde(default)]
    pub selection: StyleConfig,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct StyleConfig {
    pub fg: Option<String>,
    pub bg: Option<String>,
    #[serde(default)]
    pub modifiers: Vec<String>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            colors: ColorConfig::default(),
            file: FileColorConfig::default(),
            ui: UiColorConfig::default(),
        }
    }
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            background: "default".to_string(),
            foreground: "white".to_string(),
        }
    }
}

impl Default for FileColorConfig {
    fn default() -> Self {
        Self {
            directory: StyleConfig {
                fg: Some("blue".to_string()),
                bg: None,
                modifiers: vec!["bold".to_string()],
            },
            executable: StyleConfig {
                fg: Some("green".to_string()),
                bg: None,
                modifiers: vec!["bold".to_string()],
            },
            symlink: StyleConfig {
                fg: Some("cyan".to_string()),
                bg: None,
                modifiers: vec![],
            },
            hidden: StyleConfig {
                fg: Some("gray".to_string()),
                bg: None,
                modifiers: vec![],
            },
        }
    }
}

impl Default for UiColorConfig {
    fn default() -> Self {
        Self {
            border: StyleConfig {
                fg: Some("gray".to_string()),
                bg: None,
                modifiers: vec![],
            },
            border_focused: StyleConfig {
                fg: Some("blue".to_string()),
                bg: None,
                modifiers: vec![],
            },
            cursor: StyleConfig {
                fg: Some("white".to_string()),
                bg: Some("blue".to_string()),
                modifiers: vec!["bold".to_string()],
            },
            selection: StyleConfig {
                fg: Some("black".to_string()),
                bg: Some("yellow".to_string()),
                modifiers: vec!["bold".to_string()],
            },
        }
    }
}

fn default_background() -> String {
    "default".to_string()
}

fn default_foreground() -> String {
    "white".to_string()
}

impl StyleConfig {
    /// StyleConfigをratatuiのStyleに変換
    pub fn to_style(&self) -> Style {
        let mut style = Style::default();

        if let Some(ref fg) = self.fg {
            if let Some(color) = parse_color(fg) {
                style = style.fg(color);
            }
        }

        if let Some(ref bg) = self.bg {
            if let Some(color) = parse_color(bg) {
                style = style.bg(color);
            }
        }

        for modifier_str in &self.modifiers {
            if let Some(modifier) = parse_modifier(modifier_str) {
                style = style.add_modifier(modifier);
            }
        }

        style
    }
}

/// 色文字列をratatuiのColorに変換
fn parse_color(s: &str) -> Option<Color> {
    match s.to_lowercase().as_str() {
        "default" | "reset" => Some(Color::Reset),
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "magenta" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "gray" | "grey" => Some(Color::Gray),
        "darkgray" | "darkgrey" => Some(Color::DarkGray),
        "lightred" => Some(Color::LightRed),
        "lightgreen" => Some(Color::LightGreen),
        "lightyellow" => Some(Color::LightYellow),
        "lightblue" => Some(Color::LightBlue),
        "lightmagenta" => Some(Color::LightMagenta),
        "lightcyan" => Some(Color::LightCyan),
        "white" => Some(Color::White),
        _ => {
            // #RRGGBB形式の16進数カラーをパース
            if s.starts_with('#') && s.len() == 7 {
                let r = u8::from_str_radix(&s[1..3], 16).ok()?;
                let g = u8::from_str_radix(&s[3..5], 16).ok()?;
                let b = u8::from_str_radix(&s[5..7], 16).ok()?;
                Some(Color::Rgb(r, g, b))
            } else {
                None
            }
        }
    }
}

/// modifier文字列をratatuiのModifierに変換
fn parse_modifier(s: &str) -> Option<Modifier> {
    match s.to_lowercase().as_str() {
        "bold" => Some(Modifier::BOLD),
        "dim" => Some(Modifier::DIM),
        "italic" => Some(Modifier::ITALIC),
        "underlined" => Some(Modifier::UNDERLINED),
        "slowblink" => Some(Modifier::SLOW_BLINK),
        "rapidblink" => Some(Modifier::RAPID_BLINK),
        "reversed" => Some(Modifier::REVERSED),
        "hidden" => Some(Modifier::HIDDEN),
        "crossedout" => Some(Modifier::CROSSED_OUT),
        _ => None,
    }
}
