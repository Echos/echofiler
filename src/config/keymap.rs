use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct KeymapConfig {
    #[serde(default)]
    pub normal: HashMap<String, String>,
    #[serde(default)]
    pub visual: HashMap<String, String>,
    #[serde(default)]
    pub command: HashMap<String, String>,
    #[serde(default)]
    pub search: HashMap<String, String>,
}

impl Default for KeymapConfig {
    fn default() -> Self {
        Self {
            normal: HashMap::new(),
            visual: HashMap::new(),
            command: HashMap::new(),
            search: HashMap::new(),
        }
    }
}

impl KeymapConfig {
    /// キー表記からアクション名を取得
    pub fn get_action(&self, mode: &str, key_str: &str) -> Option<&str> {
        let map = match mode {
            "normal" => &self.normal,
            "visual" => &self.visual,
            "command" => &self.command,
            "search" => &self.search,
            _ => return None,
        };
        map.get(key_str).map(|s| s.as_str())
    }

    /// 特殊ワードかどうかを判定（<?>形式）
    pub fn is_special_word(action: &str) -> bool {
        action.starts_with('<') && action.ends_with('>')
    }

    /// プレフィックスキー後のアクションを取得
    /// prefix: 特殊ワード（例: "<bm_prefix>"）
    /// key_str: プレフィックス後のキー（例: "b"）
    pub fn get_prefix_action(&self, mode: &str, prefix: &str, key_str: &str) -> Option<&str> {
        let map = match mode {
            "normal" => &self.normal,
            "visual" => &self.visual,
            "command" => &self.command,
            "search" => &self.search,
            _ => return None,
        };

        // "<prefix> key" 形式のキーを検索
        let prefixed_key = format!("{} {}", prefix, key_str);
        map.get(&prefixed_key).map(|s| s.as_str())
    }
}
