use tokio::net::{TcpListener, TcpStream};
use tokio::io::{BufWriter, AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use bytes::BytesMut;
use std::io::Cursor;
use std::sync::Arc;
use crate::listener_thread::ListenerThread;
use crate::message::Message;
use crate::message::Request;
use crate::message::Response;

pub struct Listener {
  listener: Arc<TcpListener>,
  port: u16,
}

impl Listener {
  pub fn new(listener: TcpListener) -> Listener {
    let port = listener.local_addr().unwrap().port();
    Listener {listener: Arc::new(listener), port: port}
  }

  async fn listen(listener: Arc<TcpListener>, tx: mpsc::Sender<Message>) {
    loop {
      let tx = tx.clone();
      let (socket, _) = listener.accept().await.unwrap();

      tokio::spawn(async move {
          Listener::process(socket, tx).await;
      });
    }
  }

  pub fn get_port(&self) -> u16 {
    return self.port
  }

  pub async fn start(&mut self) -> ListenerThread {
    let listener = self.listener.clone();
    let (tx, rx) = mpsc::channel(32);
    let listener_handle = tokio::spawn(async move {
        Listener::listen(listener, tx).await;
    });
    ListenerThread::new(listener_handle, Some(rx))
  }

  // Listener: Listen for requests and send responses
  async fn process(socket: TcpStream, tx: mpsc::Sender<Message>) {
    let mut buffer = BytesMut::with_capacity(1024);
    let mut stream = BufWriter::new(socket);
    loop {
        stream.read_buf(&mut buffer).await.unwrap();
        let mut buf = Cursor::new(&buffer[..]);
        if let Ok(request) = Request::parse_message(&mut buf) {
          match request {
            Request::Ping => { 
              let (resp_tx, resp_rx) = oneshot::channel();
              let resp_tx_arc = Arc::new(resp_tx);
              tx.send(Message::new(request, resp_tx_arc)).await;
              let response = resp_rx.await;
              stream.write_buf(&mut "*0*1*1".as_bytes()).await; // Ping response
            },
            Request::RequestVote {term, candidate_id, last_log_index, last_log_term} => {
              let (resp_tx, resp_rx) = oneshot::channel();
              let resp_tx_arc = Arc::new(resp_tx);
              tx.send(Message::new(request, resp_tx_arc)).await;
              let response = resp_rx.await;
              stream.write_buf(&mut format!("*1*4*{}", candidate_id).as_bytes()).await; // Election Response
            },
            _ => println!("other"),
          }
        } 
    }
  }
}