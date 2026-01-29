use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LogConfig {
    #[serde(default = "default_level")]
    pub level: String,
    #[serde(default = "default_file")]
    pub file: String,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            file: "~/.local/share/echofiler/echofiler.log".to_string(),
        }
    }
}

fn default_level() -> String {
    "info".to_string()
}

fn default_file() -> String {
    "~/.local/share/echofiler/echofiler.log".to_string()
}
