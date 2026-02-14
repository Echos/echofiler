use anyhow::Result;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{Duration, Instant};

/// ファイル変更検知後のデバウンス間隔
const DEBOUNCE_DURATION: Duration = Duration::from_millis(500);

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    receiver: Receiver<Result<Event, notify::Error>>,
    last_event_time: Option<Instant>,
    pending: bool,
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
            last_event_time: None,
            pending: false,
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

    /// デバウンス付きイベントチェック
    /// 短時間に連続するイベントをまとめて1回のreloadにする
    pub fn check_events(&mut self) -> bool {
        // 新しいイベントがあればpendingフラグを立てる
        while self.receiver.try_recv().is_ok() {
            self.pending = true;
            self.last_event_time = Some(Instant::now());
        }

        // pendingがあり、最後のイベントからデバウンス時間が経過していればtrue
        if self.pending {
            if let Some(last) = self.last_event_time {
                if last.elapsed() >= DEBOUNCE_DURATION {
                    self.pending = false;
                    self.last_event_time = None;
                    return true;
                }
            }
        }

        false
    }

    pub fn clear_events(&mut self) {
        while self.receiver.try_recv().is_ok() {}
        self.pending = false;
        self.last_event_time = None;
    }
}
