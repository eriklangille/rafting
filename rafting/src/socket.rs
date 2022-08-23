use std::net::{SocketAddr, IpAddr};
use std::str::FromStr;

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

    pub fn to_slice(&self) -> &[SocketAddr] {
        &self.addrs[..]
    }
}