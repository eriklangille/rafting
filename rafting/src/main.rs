mod socket;

use tokio::{net::{TcpListener, TcpStream}, io::AsyncReadExt, io::AsyncWriteExt};
use bytes::BytesMut;
use socket::Socket;
use std::{thread, time};

#[tokio::main]
async fn main() {
    let ports = vec![3000, 3001];
    let mut socket_range = Socket::from_range(ports.clone());

    let listener = socket_range.bind().await;

    let listener_thread = tokio::spawn(async move {
        listen(listener).await;
    });

    match socket_range.connect().await {
        Ok(client) => {
            tokio::spawn(async move {
                process(client).await;
            });
        },
        Err(_) => println!("Could not connect :(")
    };

    listener_thread.await.unwrap();
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
        thread::sleep(time::Duration::from_secs(2));
    }
}