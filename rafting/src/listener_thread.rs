use tokio::task::JoinHandle;
use tokio::sync::mpsc::*;
use std::mem;

use crate::message::Message;

pub struct ListenerThread {
  handle: JoinHandle<()>,
  receiver: Option<Receiver<Message>>
}

impl ListenerThread {
  pub fn new(handle: JoinHandle<()>, receiver: Option<Receiver<Message>>) -> ListenerThread {
    ListenerThread { handle: handle, receiver: receiver }
  }

  pub async fn join(&mut self) {
    let handle = &mut self.handle;
    handle.await.unwrap();
  }

  pub fn get_receiver(&mut self) -> Receiver<Message> {
    mem::replace(&mut self.receiver, None).unwrap()
  }
}