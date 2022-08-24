mod socket;

use tokio::{net::{TcpListener, TcpStream}, io::AsyncReadExt, io::AsyncWriteExt};
use bytes::BytesMut;
use socket::Socket;

#[tokio::main]
async fn main() {
    let ports = vec![3000, 3001];
    let mut socket_range = Socket::from_range(ports.clone());

    let listener = socket_range.bind().await;

    tokio::spawn(async move {
        listen(listener).await;
    });
}

async fn listen(listener: TcpListener) {
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