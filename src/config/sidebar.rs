use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SidebarConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default = "default_position")]
    pub position: String,
    #[serde(default = "default_width")]
    pub width: u16,
}

impl Default for SidebarConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            position: "right".to_string(),
            width: 30,
        }
    }
}

fn default_enabled() -> bool {
    true
}

fn default_position() -> String {
    "right".to_string()
}

fn default_width() -> u16 {
    30
}
