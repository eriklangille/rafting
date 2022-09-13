use tokio::sync::{mpsc, broadcast};
use tokio::time;

use crate::internal_message::InternalMessage;
use crate::message::Message;

const TIMEOUT_RANGE: std::ops::Range<u64> = 800..1000;

pub struct ElectionTimer {
  rx: mpsc::Receiver<InternalMessage>
}

impl ElectionTimer {
  pub fn new(rx: mpsc::Receiver<InternalMessage>) -> ElectionTimer {
    ElectionTimer {rx: rx}
  }

  async fn wait_for_leader_message(&mut self) {
      // Wait for message from leader
    while let Some(msg) = self.rx.recv().await {
      match msg {
        InternalMessage::Ping => {
          return;
        },
        _ => {
          println!("uh oh :(");
        }
      }
    }
  }

  pub async fn start(&mut self, id: u16, tx: broadcast::Sender<Message>) {
      loop {
          let timeout_duration = time::Duration::from_millis(fastrand::u64(TIMEOUT_RANGE));
          let res = time::timeout(timeout_duration, self.wait_for_leader_message()).await;
          if res.is_err() {
              // TODO Call election. Timeout occurred!
              println!("Call an election");
              tx.send(Message::ElectionRequest { id: id }).unwrap(); //TODO: Handle error when not connected to any other servers
          }
      }
  }
}