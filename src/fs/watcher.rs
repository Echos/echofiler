use anyhow::Result;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    receiver: Receiver<Result<Event, notify::Error>>,
}

impl FileWatcher {
    pub fn new() -> Result<Self> {
        let (tx, rx): (Sender<Result<Event, notify::Error>>, Receiver<Result<Event, notify::Error>>) = channel();

        let watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            notify::Config::default(),
        )?;

        Ok(Self {
            watcher,
            receiver: rx,
        })
    }

    pub fn watch(&mut self, path: &Path) -> Result<()> {
        self.watcher.watch(path, RecursiveMode::NonRecursive)?;
        Ok(())
    }

    pub fn unwatch(&mut self, path: &Path) -> Result<()> {
        self.watcher.unwatch(path)?;
        Ok(())
    }

    pub fn check_events(&self) -> bool {
        // ノンブロッキングでイベントをチェック
        self.receiver.try_recv().is_ok()
    }

    pub fn clear_events(&self) {
        // すべてのイベントをクリア
        while self.receiver.try_recv().is_ok() {}
    }
}
