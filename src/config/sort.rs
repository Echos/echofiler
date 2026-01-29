use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SortConfig {
    #[serde(default = "default_method")]
    pub method: String,
    #[serde(default = "default_true")]
    pub directories_first: bool,
    #[serde(default)]
    pub reverse: bool,
}

impl Default for SortConfig {
    fn default() -> Self {
        Self {
            method: "natural".to_string(),
            directories_first: true,
            reverse: false,
        }
    }
}

fn default_method() -> String {
    "natural".to_string()
}

fn default_true() -> bool {
    true
}
