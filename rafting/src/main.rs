mod socket;
mod election_timer;
mod listener;
mod listener_thread;
mod network;
mod message;

use election_timer::ElectionTimer;
use network::Network;
use tokio::{net::{TcpStream}, io::AsyncReadExt, io::AsyncWriteExt, sync::{Mutex, broadcast}, time};
use crate::socket::Socket;
use std::{thread, sync::Arc};

type Ledger = Arc<Mutex<Vec<u64>>>;

#[tokio::main]
async fn main() {
    let ledger: Ledger = Arc::new(Mutex::new(Vec::new()));
    let current_term: Arc<u64> = Arc::new(0);

    let ports: Vec<u16> = (3000..3002).collect();
    let mut net = Network::from_slice(&ports[..]).await;

    net.listener_thread.join().await;
}
