use tokio::task::JoinHandle;
use tokio::sync::mpsc::*;
use std::mem;

pub struct ListenerThread {
  handle: JoinHandle<()>,
  receiver: Option<Receiver<u32>>
}

impl ListenerThread {
  pub fn new(handle: JoinHandle<()>, receiver: Option<Receiver<u32>>) -> ListenerThread {
    ListenerThread { handle: handle, receiver: receiver }
  }

  pub async fn join(&mut self) {
    let handle = &mut self.handle;
    handle.await.unwrap();
  }

  pub fn get_receiver(&mut self) -> Receiver<u32> {
    mem::replace(&mut self.receiver, None).unwrap()
  }
}