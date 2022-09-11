use tokio::io::AsyncWriteExt;
use tokio::io::BufWriter;
use tokio::sync::{broadcast, mpsc};
use tokio::time;

use crate::{socket};
use crate::listener_thread::ListenerThread;
use crate::message::Message;
use crate::ElectionTimer;
use crate::socket::Socket;

const ADDRESS : &'static str = "127.0.0.1";

pub struct Network {
  // connections: Arc<Mutex<Vec<mpsc::Sender<Message>>>>, // If switching to MPSC use this
  sender: broadcast::Sender<Message>,
  pub listener_thread: ListenerThread
}

impl Network { // Receiver and sender handler of the connected sockets
  pub async fn from_slice(ports: &[u16]) -> Network {
    let mut sockets = Socket::from_port_slice(ports);

    let (election_tx, election_rx) = mpsc::channel(8);
    let (tx, _rx) = broadcast::channel(8); //TODO: Wire up receiver to thread connected to other servers
    let mut election_timer = ElectionTimer::new(election_rx);

    let mut listener = sockets.bind().await;
    let mut listener_thread = listener.start().await;
    let mut rx = listener_thread.get_receiver();
    let port = listener.get_port();

    // Incoming Message handler from listeners (Remove from Network?)
    let tx2 = tx.clone();
    tokio::spawn(async move {
      while let Some(msg) = rx.recv().await {
        match msg {
          Message::Ping => {
            let _ = election_tx.send(msg).await;
          },
          Message::ElectionRequest { id } => {
            let _ = tx2.send(Message::ElectionRequest { id: id }).unwrap();
          }
          _ => {
            println!("uh oh :(");
          }
        }
      }
    });

    // Sender handler - Send outgoing messages - send requests and get responses
    let tx3 = tx.clone();
    tokio::spawn(async move {
      loop {
        match sockets.connect().await {
            Ok(client) => {
                let mut rx = tx3.subscribe();
                tokio::spawn(async move {
                    let mut stream = BufWriter::new(client);
                    while let Ok(msg) = rx.recv().await {
                      match msg {
                        Message::ElectionRequest { id } => {
                          stream.write_buf(&mut format!("*1*4*{}", id).as_bytes()).await; // Election Response
                        },
                        _ => unimplemented!()
                      }
                    }
                });
            },
            Err(socket::Error::AllConnected) => {
              break; //TODO: Respawn this thread when a server disconnects
            }
            Err(socket::Error::CouldntConnect) => {
              // Could not connect. Retry again
              time::sleep(time::Duration::from_millis(fastrand::u64(200..300))).await;
            },
        };
      }
    });

    // Election timer handler (Remove from Network)
    let tx4 = tx.clone();
    tokio::spawn(async move {
        election_timer.start(port, tx4).await;
    });

    Network {sender: tx, listener_thread: listener_thread}
  }

  pub fn get_tx_copy(&self) -> broadcast::Sender<Message> {
    return self.sender.clone();
  }
}