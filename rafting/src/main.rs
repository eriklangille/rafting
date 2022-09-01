mod socket;
mod election_timer;
mod listener;
mod listener_thread;

use election_timer::ElectionTimer;
use tokio::{net::{TcpStream}, io::AsyncReadExt, io::AsyncWriteExt, sync::{Mutex, broadcast}, time};
use bytes::BytesMut;
use socket::Socket;
use std::{thread, sync::Arc};

type Ledger = Arc<Mutex<Vec<u64>>>;

#[tokio::main]
async fn main() {
    let ledger: Ledger = Arc::new(Mutex::new(Vec::new()));
    let current_term: Arc<u64> = Arc::new(0);

    let mut socket_range = Socket::from_vector(vec![3000, 3001]);
    let mut listener = socket_range.bind().await;
    let mut listener_thread = listener.start().await;
    let mut election_timer = ElectionTimer::new(listener_thread.get_receiver());

    let (tx, _rx) = broadcast::channel(8);

    let port = socket_range.get_port().unwrap();

    match socket_range.connect().await {
        Ok(client) => {
            let rx = tx.subscribe();
            tokio::spawn(async move {
                process_writefirst(client, rx).await;
            });
        },
        Err(_) => println!("Could not connect :(")
    };

    tokio::spawn(async move {
        election_timer.start(tx).await;
    });

    listener_thread.join().await;
}


async fn process_writefirst(mut socket: TcpStream, mut rx: broadcast::Receiver<u32>) {
    // Do something
    let mut buf = BytesMut::with_capacity(10);
    loop {
        match rx.recv().await.unwrap() {
            0 => {
                // Call election
                socket.write_all(format!("*{}*", "0").as_bytes()).await.unwrap();
                socket.read_buf(&mut buf).await.unwrap();
                println!("GOT = {:?}", buf);
                buf.clear();
            },
            _ => println!("Uh oh not implemented"),
        }
    }
}