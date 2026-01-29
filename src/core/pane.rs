use super::Tab;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneSide {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Pane {
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
}

impl Pane {
    pub fn new(initial_path: std::path::PathBuf) -> Self {
        Self::with_show_hidden(initial_path, false)
    }

    pub fn with_show_hidden(initial_path: std::path::PathBuf, show_hidden: bool) -> Self {
        Self {
            tabs: vec![Tab::with_show_hidden(initial_path, show_hidden)],
            active_tab: 0,
        }
    }

    pub fn current_tab(&self) -> &Tab {
        &self.tabs[self.active_tab]
    }

    pub fn current_tab_mut(&mut self) -> &mut Tab {
        &mut self.tabs[self.active_tab]
    }

    pub fn new_tab(&mut self, path: std::path::PathBuf, show_hidden: bool) {
        self.tabs.push(Tab::with_show_hidden(path, show_hidden));
        self.active_tab = self.tabs.len() - 1;
    }

    pub fn close_tab(&mut self) {
        if self.tabs.len() > 1 {
            self.tabs.remove(self.active_tab);
            if self.active_tab >= self.tabs.len() {
                self.active_tab = self.tabs.len() - 1;
            }
        }
    }

    pub fn next_tab(&mut self) {
        if self.tabs.len() > 1 {
            self.active_tab = (self.active_tab + 1) % self.tabs.len();
        }
    }

    pub fn prev_tab(&mut self) {
        if self.tabs.len() > 1 {
            if self.active_tab == 0 {
                self.active_tab = self.tabs.len() - 1;
            } else {
                self.active_tab -= 1;
            }
        }
    }
}
