use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LayoutConfig {
    #[serde(default = "default_style")]
    pub style: String,
    #[serde(default = "default_ratio")]
    pub ratio: Vec<u16>,
    #[serde(default)]
    pub show_preview: bool,
    #[serde(default = "default_preview_ratio")]
    pub preview_ratio: u16,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            style: "dual".to_string(),
            ratio: vec![1, 1],
            show_preview: false,
            preview_ratio: 3,
        }
    }
}

fn default_style() -> String {
    "dual".to_string()
}

fn default_ratio() -> Vec<u16> {
    vec![1, 1]
}

fn default_preview_ratio() -> u16 {
    3
}
