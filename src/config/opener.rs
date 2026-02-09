use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct OpenerConfig {
    /// デフォルトのオープナーコマンド（xdg-open等）
    #[serde(default = "default_opener")]
    pub default: String,

    /// 拡張子ごとのカスタムオープナー
    #[serde(default)]
    pub extension: HashMap<String, String>,

    /// MIMEタイプごとのカスタムオープナー
    #[serde(default)]
    pub mime: HashMap<String, String>,

    /// エディタコマンド（テキストファイル用）
    #[serde(default = "default_editor")]
    pub editor: String,

    /// ページャコマンド（テキストファイル閲覧用）
    #[serde(default = "default_pager")]
    pub pager: String,
}

impl Default for OpenerConfig {
    fn default() -> Self {
        Self {
            default: default_opener(),
            extension: HashMap::new(),
            mime: HashMap::new(),
            editor: default_editor(),
            pager: default_pager(),
        }
    }
}

fn default_opener() -> String {
    if cfg!(target_os = "macos") {
        "open".to_string()
    } else if cfg!(target_os = "windows") {
        "cmd /c start \"\"".to_string()
    } else {
        "xdg-open".to_string()
    }
}

fn default_editor() -> String {
    std::env::var("EDITOR").unwrap_or_else(|_| {
        if cfg!(target_os = "windows") {
            "notepad".to_string()
        } else {
            "vi".to_string()
        }
    })
}

fn default_pager() -> String {
    std::env::var("PAGER").unwrap_or_else(|_| {
        if cfg!(target_os = "windows") {
            "more".to_string()
        } else {
            "less".to_string()
        }
    })
}
