mod socket;
mod election_timer;

use election_timer::ElectionTimer;
use tokio::{net::{TcpListener, TcpStream}, io::AsyncReadExt, io::AsyncWriteExt, sync::Mutex, sync::mpsc, time};
use bytes::BytesMut;
use socket::Socket;
use std::{thread, sync::Arc};
use fastrand;

type Ledger = Arc<Mutex<Vec<u64>>>;

#[tokio::main]
async fn main() {
    let ledger: Ledger = Arc::new(Mutex::new(Vec::new()));
    let current_term: Arc<u64> = Arc::new(0);
    let (tx, mut rx) = mpsc::channel(32);
    let mut socket_range = Socket::from_vector(vec![3000, 3001]);
    let mut election_timer = ElectionTimer::new(rx);

    let listener = socket_range.bind().await;

    let listener_thread = tokio::spawn(async move {
        listen(listener).await;
    });

    match socket_range.connect().await {
        Ok(client) => {
            tokio::spawn(async move {
                process_writefirst(client).await;
            });
        },
        Err(_) => println!("Could not connect :(")
    };

    tokio::spawn(async move {
        election_timer.start().await;
    });

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
        thread::sleep(time::Duration::from_millis(fastrand::u64(200..300)));
    }
}

async fn process_writefirst(mut socket: TcpStream) {
    // Do something
    let mut buf = BytesMut::with_capacity(10);
    loop {
        socket.write_all(b"hello world\n").await.unwrap();
        thread::sleep(time::Duration::from_secs(2));
        socket.read_buf(&mut buf).await.unwrap();
        println!("GOT = {:?}", buf);
        buf.clear();
    }
}