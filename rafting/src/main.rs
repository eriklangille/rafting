mod socket;

use tokio::{net::{TcpListener, TcpStream}, io::AsyncReadExt, io::AsyncWriteExt};
use bytes::BytesMut;
use socket::Socket;

#[tokio::main]
async fn main() {
    let socket_range = Socket::from_range(vec![3000, 3001, 3002]);
    let listener = TcpListener::bind(socket_range.to_slice()).await.unwrap();
    let port = listener.local_addr().unwrap().port();

    println!("Listening on {:?}", port);

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