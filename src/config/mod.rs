pub mod general;
pub mod keymap;
pub mod layout;
pub mod log;
pub mod opener;
pub mod preview;
pub mod sidebar;
pub mod sort;
pub mod theme;
pub mod watcher;

use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

const DEFAULT_CONFIG: &str = include_str!("../../config/default/echofiler.toml");
const DEFAULT_THEME: &str = include_str!("../../config/default/theme.toml");
const DEFAULT_KEYMAP: &str = include_str!("../../config/default/keymap.toml");
const DEFAULT_OPENER: &str = include_str!("../../config/default/opener.toml");

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub general: general::GeneralConfig,
    #[serde(default)]
    pub layout: layout::LayoutConfig,
    #[serde(default)]
    pub sort: sort::SortConfig,
    #[serde(default)]
    pub preview: preview::PreviewConfig,
    #[serde(default)]
    pub log: log::LogConfig,
    #[serde(default)]
    pub keymap: keymap::KeymapConfig,
    #[serde(default)]
    pub theme: theme::ThemeConfig,
    #[serde(default)]
    pub sidebar: sidebar::SidebarConfig,
    #[serde(default)]
    pub opener: opener::OpenerConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: general::GeneralConfig::default(),
            layout: layout::LayoutConfig::default(),
            sort: sort::SortConfig::default(),
            preview: preview::PreviewConfig::default(),
            log: log::LogConfig::default(),
            keymap: keymap::KeymapConfig::default(),
            theme: theme::ThemeConfig::default(),
            sidebar: sidebar::SidebarConfig::default(),
            opener: opener::OpenerConfig::default(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path();
        let theme_path = Self::get_theme_path();
        let keymap_path = Self::get_keymap_path();

        let mut config: Config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            toml::from_str(&content)?
        } else {
            toml::from_str(DEFAULT_CONFIG)?
        };

        // テーマファイルを読み込み
        config.theme = if theme_path.exists() {
            let content = fs::read_to_string(&theme_path)?;
            toml::from_str(&content)?
        } else {
            toml::from_str(DEFAULT_THEME)?
        };

        // キーマップファイルを読み込み
        config.keymap = if keymap_path.exists() {
            let content = fs::read_to_string(&keymap_path)?;
            toml::from_str(&content)?
        } else {
            toml::from_str(DEFAULT_KEYMAP)?
        };

        // オープナーファイルを読み込み
        let opener_path = Self::get_opener_path();
        config.opener = if opener_path.exists() {
            let content = fs::read_to_string(&opener_path)?;
            toml::from_str(&content)?
        } else {
            toml::from_str(DEFAULT_OPENER)?
        };

        Ok(config)
    }

    pub fn get_config_path() -> PathBuf {
        if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(config_home).join("echofiler/echofiler.toml")
        } else if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".config/echofiler/echofiler.toml")
        } else {
            PathBuf::from("echofiler.toml")
        }
    }

    pub fn get_theme_path() -> PathBuf {
        if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(config_home).join("echofiler/theme.toml")
        } else if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".config/echofiler/theme.toml")
        } else {
            PathBuf::from("theme.toml")
        }
    }

    pub fn get_keymap_path() -> PathBuf {
        if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(config_home).join("echofiler/keymap.toml")
        } else if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".config/echofiler/keymap.toml")
        } else {
            PathBuf::from("keymap.toml")
        }
    }

    pub fn get_opener_path() -> PathBuf {
        if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(config_home).join("echofiler/opener.toml")
        } else if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".config/echofiler/opener.toml")
        } else {
            PathBuf::from("opener.toml")
        }
    }
}

// 設定ファイルパスを取得する関数を公開
pub fn get_config_path() -> PathBuf {
    Config::get_config_path()
}

pub fn get_theme_path() -> PathBuf {
    Config::get_theme_path()
}

pub fn get_keymap_path() -> PathBuf {
    Config::get_keymap_path()
}

pub fn get_opener_path() -> PathBuf {
    Config::get_opener_path()
}
