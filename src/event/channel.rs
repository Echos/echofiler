use tokio::sync::mpsc;

pub type Sender<T> = mpsc::UnboundedSender<T>;
pub type Receiver<T> = mpsc::UnboundedReceiver<T>;

pub fn create_channel<T>() -> (Sender<T>, Receiver<T>) {
    mpsc::unbounded_channel()
}
