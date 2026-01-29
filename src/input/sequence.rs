use super::Key;
use std::time::{Duration, Instant};

pub struct KeyBuffer {
    keys: Vec<Key>,
    timeout: Duration,
    last_input: Instant,
}

impl Default for KeyBuffer {
    fn default() -> Self {
        Self {
            keys: Vec::new(),
            timeout: Duration::from_millis(1000),
            last_input: Instant::now(),
        }
    }
}

impl KeyBuffer {
    pub fn push(&mut self, key: Key) {
        if self.last_input.elapsed() > self.timeout {
            self.keys.clear();
        }
        self.keys.push(key);
        self.last_input = Instant::now();
    }

    pub fn clear(&mut self) {
        self.keys.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
}
