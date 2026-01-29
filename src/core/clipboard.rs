use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum ClipboardMode {
    Copy,
    Cut,
}

#[derive(Debug, Clone)]
pub struct Clipboard {
    pub paths: Vec<PathBuf>,
    pub mode: Option<ClipboardMode>,
}

impl Default for Clipboard {
    fn default() -> Self {
        Self {
            paths: Vec::new(),
            mode: None,
        }
    }
}

impl Clipboard {
    pub fn clear(&mut self) {
        self.paths.clear();
        self.mode = None;
    }

    pub fn is_empty(&self) -> bool {
        self.paths.is_empty()
    }
}
