use std::net::{SocketAddr, IpAddr};

use tokio::{net::{TcpListener, TcpStream}, io::AsyncReadExt, io::AsyncWriteExt};
use bytes::BytesMut;
use std::str::FromStr;

const ADDRESS : &'static str = "127.0.0.1";

struct Socket {
    addrs: Vec<SocketAddr>,
}

impl Socket {
    fn from_range(ports: Vec<u16>) -> Socket {
        let addrs : Vec<SocketAddr> = ports.clone().into_iter()
        .map(|port| SocketAddr::new(IpAddr::from_str(ADDRESS).unwrap(), port)).collect();
        Socket {addrs: addrs}
    }

    pub fn to_slice(&self) -> &[SocketAddr] {
        &self.addrs[..]
    }
}

#[tokio::main]
async fn main() {
    let socket_range = Socket::from_range(vec![3000, 3001, 3002]);
    let listener = TcpListener::bind(socket_range.to_slice()).await.unwrap();

    println!("Listening");

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(mut socket: TcpStream) {
    // Do something
    let mut buf = BytesMut::with_capacity(10);
    loop {
        socket.read_buf(&mut buf).await.unwrap();
        println!("GOT = {:?}", buf);
        buf.clear();
        socket.write_all(b"buf\n").await.unwrap();
    }
}