mod socket;
mod election_timer;
mod listener;

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

    let mut listener = socket_range.bind().await;
    let port = socket_range.get_port().unwrap();

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

    let listener_thread = listener.start().await;

    listener_thread.await.unwrap();
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