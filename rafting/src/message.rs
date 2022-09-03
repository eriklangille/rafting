use std::io::Cursor;
use bytes::Buf;

#[derive(Debug)]
#[derive(Clone)]
pub enum Message {
  Ping,
  Election {sender: u16},
}

impl Message {
  pub fn parse_message(src: &mut Cursor<&[u8]>) -> Option<Message> {
    match src.get_u8() {
      b'*' => {
        println!("msg")
      },
      _ => println!("ERROR")
    }
    Some(Message::Ping)
  }
}