use std::net::{SocketAddr, IpAddr};
use std::str::FromStr;

use tokio::net::TcpListener;

const ADDRESS : &'static str = "127.0.0.1";

pub struct Socket {
    addrs: Vec<SocketAddr>,
}

impl Socket {
    pub fn from_range(ports: Vec<u16>) -> Socket {
        let addrs : Vec<SocketAddr> = ports.clone().into_iter()
        .map(|port| SocketAddr::new(IpAddr::from_str(ADDRESS).unwrap(), port)).collect();
        Socket {addrs: addrs}
    }

    fn to_slice(&self) -> &[SocketAddr] {
        &self.addrs[..]
    }

    pub async fn bind(&mut self) -> TcpListener {
      let listener = TcpListener::bind(self.to_slice()).await.unwrap();

      let port = listener.local_addr().unwrap().port();

      println!("Listening on {:?}", port);

      self.remove_addr(port);

      return listener;
    }

    fn remove_addr(&mut self, port: u16) {
      self.addrs = self.addrs.clone().into_iter().filter(|socket| socket.port() != port).collect();
    }
}