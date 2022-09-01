use tokio::net::{TcpListener, TcpStream};
use tokio::time;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use bytes::BytesMut;
use std::sync::Arc;
use crate::listener_thread::ListenerThread;

pub struct Listener {
  listener: Arc<TcpListener>,
}

impl Listener {
  pub fn new(listener: TcpListener) -> Listener {
    Listener {listener: Arc::new(listener)}
  }

  async fn listen(listener: Arc<TcpListener>, tx: mpsc::Sender<u32>) {
    loop {
      let tx = tx.clone();
      let (socket, _) = listener.accept().await.unwrap();

      tokio::spawn(async move {
          Listener::process(socket, tx).await;
      });
    }
  }

  pub async fn start(&mut self) -> ListenerThread {
    let listener = self.listener.clone();
    let (tx, rx) = mpsc::channel(32);
    let listener_handle = tokio::spawn(async move {
        Listener::listen(listener, tx).await;
    });
    ListenerThread::new(listener_handle, Some(rx))
  }

  async fn process(mut socket: TcpStream, tx: mpsc::Sender<u32>) {
    // Do something
    let mut buf = BytesMut::with_capacity(10);
    loop {
        socket.read_buf(&mut buf).await.unwrap();
        println!("GOT = {:?}", buf);
        tx.send(0).await.unwrap();
        buf.clear();
        socket.write_all(b"buf\n").await.unwrap();
        time::sleep(time::Duration::from_millis(fastrand::u64(200..300))).await;
    }
  }
}