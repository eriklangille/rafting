use tokio::task::JoinHandle;
use tokio::sync::mpsc::*;

pub struct ListenerThread {
  handle: JoinHandle<()>,
  receiver: Receiver<u32>
}

impl ListenerThread {
  pub async fn join(&mut self) {
    let handle = &mut self.handle;
    handle.await.unwrap();
  }

  pub fn get_receiver(&mut self) -> &Receiver<u32> {
    return &self.receiver;
  }
}