use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PreviewConfig {
    #[serde(default = "default_max_size")]
    pub max_size: String,
    #[serde(default = "default_true")]
    pub syntax_highlight: bool,
    #[serde(default = "default_image_protocol")]
    pub image_protocol: String,
}

impl Default for PreviewConfig {
    fn default() -> Self {
        Self {
            max_size: "10MB".to_string(),
            syntax_highlight: true,
            image_protocol: "auto".to_string(),
        }
    }
}

fn default_max_size() -> String {
    "10MB".to_string()
}

fn default_true() -> bool {
    true
}

fn default_image_protocol() -> String {
    "auto".to_string()
}
