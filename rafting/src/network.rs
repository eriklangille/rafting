use std::sync::Arc;
use std::net::{SocketAddr, IpAddr};
use std::str::FromStr;

use tokio::sync::{broadcast, mpsc, Mutex};
use tokio::net::{TcpListener, TcpStream};

use crate::listener;
use crate::message::Message;

const ADDRESS : &'static str = "127.0.0.1";

pub struct Network {
  connections: Arc<Mutex<Vec<broadcast::Sender<Message>>>>,
}

impl Network {
  pub async fn from_slice(ports: &[u16]) -> Network {
    let addrs : Vec<SocketAddr> = ports.into_iter()
    .map(|port| SocketAddr::new(IpAddr::from_str(ADDRESS).unwrap(), *port))
    .collect();

    let listener = TcpListener::bind(&addrs[..]).await.unwrap();

    let listener_thread = listener::Listener::new(listener);

    // TODO: Message processor that uses the right sender depending on the contents of the message
    
    Network {connections: Arc::new(Mutex::new(Vec::new()))}
  }
}