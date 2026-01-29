use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Selection {
    pub indices: HashSet<usize>,
}

impl Selection {
    pub fn toggle(&mut self, index: usize) {
        if !self.indices.remove(&index) {
            self.indices.insert(index);
        }
    }

    pub fn clear(&mut self) {
        self.indices.clear();
    }

    pub fn is_selected(&self, index: usize) -> bool {
        self.indices.contains(&index)
    }
}
