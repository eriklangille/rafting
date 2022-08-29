use tokio::net::{TcpListener, TcpStream};
use tokio::time;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::task::JoinHandle;
use tokio::sync::mpsc::*;
use tokio::sync::mpsc;
use bytes::BytesMut;
use std::sync::Arc;
use crate::listener_thread::ListenerThread;

pub struct Listener {
  listener: Arc<TcpListener>,
  sender: Option<Sender<u32>>,
}

impl Listener {
  pub fn new(listener: TcpListener) -> Listener {
    Listener {listener: Arc::new(listener), sender: None}
  }

  async fn listen(listener: Arc<TcpListener>) {
    loop {
      let (socket, _) = listener.accept().await.unwrap();

      tokio::spawn(async move {
          Listener::process(socket).await;
      });
    }
  }

  pub async fn start(&mut self) -> ListenerThread {
    let listener = self.listener.clone();
    let (tx, mut rx) = mpsc::channel(32);
    self.sender = Some(tx);
    let listener_handle = tokio::spawn(async move {
        Listener::listen(listener).await;
    });
    ListenerThread {handle: listener_handle, receiver: rx}
  }

  async fn process(mut socket: TcpStream) {
    // Do something
    let mut buf = BytesMut::with_capacity(10);
    loop {
        socket.read_buf(&mut buf).await.unwrap();
        println!("GOT = {:?}", buf);
        buf.clear();
        socket.write_all(b"buf\n").await.unwrap();
        time::sleep(time::Duration::from_millis(fastrand::u64(200..300))).await;
    }
  }
}