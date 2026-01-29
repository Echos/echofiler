use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IconStyle {
    NerdFonts,
    Emoji,
    Ascii,
}

impl Default for IconStyle {
    fn default() -> Self {
        IconStyle::NerdFonts
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GeneralConfig {
    #[serde(default)]
    pub show_hidden: bool,
    #[serde(default = "default_true")]
    pub confirm_delete: bool,
    #[serde(default = "default_true")]
    pub confirm_overwrite: bool,
    #[serde(default)]
    pub use_trash: bool,
    #[serde(default)]
    pub show_icons: bool,
    #[serde(default)]
    pub icon_style: IconStyle,
    #[serde(default = "default_icon_spacing")]
    pub icon_spacing: u8,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            show_hidden: false,
            confirm_delete: true,
            confirm_overwrite: true,
            use_trash: false,
            show_icons: false,
            icon_style: IconStyle::default(),
            icon_spacing: 1,
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_icon_spacing() -> u8 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_style_default() {
        let style = IconStyle::default();
        assert_eq!(style, IconStyle::NerdFonts);
    }

    #[test]
    fn test_general_config_default() {
        let config = GeneralConfig::default();

        assert_eq!(config.show_hidden, false);
        assert_eq!(config.confirm_delete, true);
        assert_eq!(config.confirm_overwrite, true);
        assert_eq!(config.use_trash, false);
        assert_eq!(config.show_icons, false);
        assert_eq!(config.icon_style, IconStyle::NerdFonts);
        assert_eq!(config.icon_spacing, 1);
    }

    #[test]
    fn test_general_config_deserialize() {
        let toml_str = r#"
            show_hidden = true
            confirm_delete = false
            confirm_overwrite = false
            use_trash = true
            show_icons = true
            icon_style = "emoji"
            icon_spacing = 2
        "#;

        let config: GeneralConfig = toml::from_str(toml_str).unwrap();

        assert_eq!(config.show_hidden, true);
        assert_eq!(config.confirm_delete, false);
        assert_eq!(config.confirm_overwrite, false);
        assert_eq!(config.use_trash, true);
        assert_eq!(config.show_icons, true);
        assert_eq!(config.icon_style, IconStyle::Emoji);
        assert_eq!(config.icon_spacing, 2);
    }

    #[test]
    fn test_icon_style_deserialize() {
        let toml_str = r#"icon_style = "nerd-fonts""#;
        let config: toml::Value = toml::from_str(toml_str).unwrap();
        let style: IconStyle = config.get("icon_style").unwrap().clone().try_into().unwrap();
        assert_eq!(style, IconStyle::NerdFonts);

        let toml_str = r#"icon_style = "emoji""#;
        let config: toml::Value = toml::from_str(toml_str).unwrap();
        let style: IconStyle = config.get("icon_style").unwrap().clone().try_into().unwrap();
        assert_eq!(style, IconStyle::Emoji);

        let toml_str = r#"icon_style = "ascii""#;
        let config: toml::Value = toml::from_str(toml_str).unwrap();
        let style: IconStyle = config.get("icon_style").unwrap().clone().try_into().unwrap();
        assert_eq!(style, IconStyle::Ascii);
    }

    #[test]
    fn test_general_config_partial_deserialize() {
        // 一部のフィールドのみ指定した場合、残りはデフォルト値になる
        let toml_str = r#"
            show_hidden = true
        "#;

        let config: GeneralConfig = toml::from_str(toml_str).unwrap();

        assert_eq!(config.show_hidden, true);
        assert_eq!(config.confirm_delete, true); // デフォルト
        assert_eq!(config.show_icons, false); // デフォルト
    }
}
