use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl From<KeyEvent> for Key {
    fn from(event: KeyEvent) -> Self {
        Self {
            code: event.code,
            modifiers: event.modifiers,
        }
    }
}

impl Key {
    pub fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }

    pub fn char(c: char) -> Self {
        Self {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
        }
    }

    /// キーをキーマップ用の文字列表記に変換
    pub fn to_keymap_string(&self) -> String {
        let key_str = match self.code {
            // スペースは " " ではなく "Space" と表記する（keymap.tomlでの可読性のため）
            KeyCode::Char(' ') => "Space".to_string(),
            KeyCode::Char(c) => c.to_string(),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Esc => "Escape".to_string(),
            KeyCode::Backspace => "Backspace".to_string(),
            KeyCode::Tab => "Tab".to_string(),
            KeyCode::Delete => "Delete".to_string(),
            KeyCode::Insert => "Insert".to_string(),
            KeyCode::Home => "Home".to_string(),
            KeyCode::End => "End".to_string(),
            KeyCode::PageUp => "PageUp".to_string(),
            KeyCode::PageDown => "PageDown".to_string(),
            KeyCode::Up => "Up".to_string(),
            KeyCode::Down => "Down".to_string(),
            KeyCode::Left => "Left".to_string(),
            KeyCode::Right => "Right".to_string(),
            KeyCode::F(n) => format!("F{}", n),
            _ => return String::new(),
        };

        // モディファイアを追加
        if self.modifiers.contains(KeyModifiers::CONTROL) {
            format!("<C-{}>", key_str)
        } else if self.modifiers.contains(KeyModifiers::ALT) {
            format!("<A-{}>", key_str)
        } else if self.modifiers.contains(KeyModifiers::SHIFT)
            && matches!(self.code, KeyCode::Char(c) if c != ' ')
        {
            // Shiftは大文字で表現（Charの場合のみ。Spaceは "SPACE" にしない）
            key_str.to_uppercase()
        } else {
            key_str
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_creation() {
        let key = Key::new(KeyCode::Char('a'), KeyModifiers::NONE);
        assert_eq!(key.code, KeyCode::Char('a'));
        assert_eq!(key.modifiers, KeyModifiers::NONE);
    }

    #[test]
    fn test_key_char() {
        let key = Key::char('x');
        assert_eq!(key.code, KeyCode::Char('x'));
        assert_eq!(key.modifiers, KeyModifiers::NONE);
    }

    #[test]
    fn test_key_from_event() {
        let event = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL);
        let key = Key::from(event);
        assert_eq!(key.code, KeyCode::Char('a'));
        assert_eq!(key.modifiers, KeyModifiers::CONTROL);
    }

    #[test]
    fn test_to_keymap_string_simple_char() {
        let key = Key::char('j');
        assert_eq!(key.to_keymap_string(), "j");
    }

    #[test]
    fn test_to_keymap_string_space() {
        // keymap.toml では "Space" と書けるようにする
        assert_eq!(Key::char(' ').to_keymap_string(), "Space");
        assert_eq!(
            Key::new(KeyCode::Char(' '), KeyModifiers::SHIFT).to_keymap_string(),
            "Space"
        );
        assert_eq!(
            Key::new(KeyCode::Char(' '), KeyModifiers::CONTROL).to_keymap_string(),
            "<C-Space>"
        );
    }

    #[test]
    fn test_to_keymap_string_uppercase() {
        let key = Key::new(KeyCode::Char('j'), KeyModifiers::SHIFT);
        assert_eq!(key.to_keymap_string(), "J");
    }

    #[test]
    fn test_to_keymap_string_control() {
        let key = Key::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert_eq!(key.to_keymap_string(), "<C-c>");
    }

    #[test]
    fn test_to_keymap_string_alt() {
        let key = Key::new(KeyCode::Char('x'), KeyModifiers::ALT);
        assert_eq!(key.to_keymap_string(), "<A-x>");
    }

    #[test]
    fn test_to_keymap_string_special_keys() {
        assert_eq!(
            Key::new(KeyCode::Enter, KeyModifiers::NONE).to_keymap_string(),
            "Enter"
        );
        assert_eq!(
            Key::new(KeyCode::Esc, KeyModifiers::NONE).to_keymap_string(),
            "Escape"
        );
        assert_eq!(
            Key::new(KeyCode::Tab, KeyModifiers::NONE).to_keymap_string(),
            "Tab"
        );
        assert_eq!(
            Key::new(KeyCode::Backspace, KeyModifiers::NONE).to_keymap_string(),
            "Backspace"
        );
        assert_eq!(
            Key::new(KeyCode::Delete, KeyModifiers::NONE).to_keymap_string(),
            "Delete"
        );
    }

    #[test]
    fn test_to_keymap_string_arrow_keys() {
        assert_eq!(
            Key::new(KeyCode::Up, KeyModifiers::NONE).to_keymap_string(),
            "Up"
        );
        assert_eq!(
            Key::new(KeyCode::Down, KeyModifiers::NONE).to_keymap_string(),
            "Down"
        );
        assert_eq!(
            Key::new(KeyCode::Left, KeyModifiers::NONE).to_keymap_string(),
            "Left"
        );
        assert_eq!(
            Key::new(KeyCode::Right, KeyModifiers::NONE).to_keymap_string(),
            "Right"
        );
    }

    #[test]
    fn test_to_keymap_string_function_keys() {
        assert_eq!(
            Key::new(KeyCode::F(1), KeyModifiers::NONE).to_keymap_string(),
            "F1"
        );
        assert_eq!(
            Key::new(KeyCode::F(12), KeyModifiers::NONE).to_keymap_string(),
            "F12"
        );
    }

    #[test]
    fn test_key_equality() {
        let key1 = Key::char('a');
        let key2 = Key::char('a');
        let key3 = Key::char('b');

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }
}
