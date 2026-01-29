use anyhow::Result;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher as NotifyWatcher};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};

/// 設定ファイル監視
pub struct ConfigWatcher {
    _watcher: RecommendedWatcher,
    receiver: Receiver<Result<Event, notify::Error>>,
}

impl ConfigWatcher {
    /// 新しい設定ファイル監視を作成
    pub fn new() -> Result<Self> {
        let (tx, rx) = channel();

        let watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            notify::Config::default(),
        )?;

        Ok(Self {
            _watcher: watcher,
            receiver: rx,
        })
    }

    /// 設定ファイルを監視対象に追加
    pub fn watch(&mut self, path: &PathBuf) -> Result<()> {
        self._watcher.watch(path, RecursiveMode::NonRecursive)?;
        Ok(())
    }

    /// 設定ファイルの変更をチェック（ノンブロッキング）
    pub fn check_changes(&self) -> bool {
        self.receiver.try_recv().is_ok()
    }

    /// イベントをクリア
    pub fn clear_events(&self) {
        while self.receiver.try_recv().is_ok() {}
    }
}
