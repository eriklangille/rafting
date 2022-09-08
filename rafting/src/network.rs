use std::sync::Arc;
use std::net::{SocketAddr, IpAddr};
use std::str::FromStr;

use tokio::sync::{broadcast, mpsc, Mutex};
use tokio::net::{TcpListener, TcpStream};

use crate::listener;
use crate::message::Message;
use crate::ElectionTimer;

const ADDRESS : &'static str = "127.0.0.1";

pub struct Network {
  connections: Arc<Mutex<Vec<broadcast::Sender<Message>>>>,
}

impl Network {
  pub async fn from_slice(ports: &[u16]) -> Network {
    let addrs : Vec<SocketAddr> = ports.into_iter()
    .map(|port| SocketAddr::new(IpAddr::from_str(ADDRESS).unwrap(), *port))
    .collect();

    let (election_tx, election_rx) = mpsc::channel(8);
    let (tx, _rx) = broadcast::channel(8); //TODO: Wire up receiver to thread connected to other servers
    let mut election_timer = ElectionTimer::new(election_rx);

    let tcp_listener = TcpListener::bind(&addrs[..]).await.unwrap();

    let mut listener = listener::Listener::new(tcp_listener);
    let mut listener_thread = listener.start().await;
    let mut rx = listener_thread.get_receiver();

    tokio::spawn(async move {
      while let Some(msg) = rx.recv().await {
        match msg {
          Message::Ping => {
            let _ = election_tx.send(msg).await;
          },
          _ => {
            println!("uh oh :(");
          }
        }
      }
    });

    tokio::spawn(async move {
        election_timer.start(tx).await;
    });

    // TODO: Message processor that uses the right sender depending on the contents of the message
    
    Network {connections: Arc::new(Mutex::new(Vec::new()))}
  }
}