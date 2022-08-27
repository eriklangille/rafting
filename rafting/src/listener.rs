use tokio::net::{TcpListener, TcpStream};
use tokio::time;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::task::JoinHandle;
use bytes::BytesMut;
use std::sync::Arc;

pub struct Listener {
  listener: Arc<TcpListener>,
}

impl Listener {
  pub fn new(listener: TcpListener) -> Listener {
    Listener {listener: Arc::new(listener)}
  }

  async fn listen(listener: Arc<TcpListener>) {
    loop {
      let (socket, _) = listener.accept().await.unwrap();

      tokio::spawn(async move {
          Listener::process(socket).await;
      });
    }
  }

  pub async fn start(&mut self) -> JoinHandle<()> {
    let listener = self.listener.clone();
    let listener_thread = tokio::spawn(async move {
        Listener::listen(listener).await;
    });
    listener_thread
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