use std::net::{SocketAddr, IpAddr};
use std::str::FromStr;

use tokio::net::{TcpListener, TcpStream};
use crate::listener::Listener;

const ADDRESS : &'static str = "127.0.0.1";

pub struct Socket {
    addrs: Vec<SocketAddr>,
    port: Option<u16>,
    connections: Vec<u16>,
}

impl Socket {
    pub fn from_vector(ports: Vec<u16>) -> Socket {
        let addrs : Vec<SocketAddr> = ports.clone().into_iter()
        .map(|port| SocketAddr::new(IpAddr::from_str(ADDRESS).unwrap(), port)).collect();
        Socket {addrs: addrs, port: None, connections: Vec::new()}
    }

    fn to_slice(&self) -> &[SocketAddr] {
        &self.addrs[..]
    }

    pub fn get_port(&self) -> Option<u16> {
      self.port.clone()
    }

    pub async fn bind(&mut self) -> Listener {
      let listener = TcpListener::bind(self.to_slice()).await.unwrap();

      let port = listener.local_addr().unwrap().port();
      self.port = Some(port);

      println!("Listening on {:?}", port);

      self.remove_addr(port);

      Listener::new(listener)
    }

    pub async fn connect(&mut self) -> Result<TcpStream, ()> {
      let client = match TcpStream::connect(self.to_slice()).await {
        Ok(client) => client,
        Err(_e) => return Err(()),
      };
      let port = client.peer_addr().unwrap().port();
      self.connections.push(port);

      println!("Connected to {:?}", port);

      self.remove_addr(port);

      Ok(client)
    }

    fn remove_addr(&mut self, port: u16) {
      self.addrs = self.addrs.clone().into_iter().filter(|socket| socket.port() != port).collect();
    }
}