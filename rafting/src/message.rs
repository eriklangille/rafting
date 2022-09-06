use std::{io::Cursor};
use bytes::Buf;
use atoi::FromRadix10;

#[derive(Debug)]
#[derive(Clone)]
pub enum Message {
  Ping,
  Election {sender: u16},
}

pub enum Error {
  Incomplete,
}

impl Message {
  pub fn parse_message(src: &mut Cursor<&[u8]>) -> Result<Message, Error> {
    let mut message_type = None;
    let mut content_length = None;
    loop {
      let bytes = 0;
      match get_u8(src)? {
        b'*' => {
          if message_type == None {
            message_type = Some(parse_int(src)?);
          } else if content_length == None {
            content_length = Some(parse_int(src)?);
          } else {
            let start_pos = src.position() as u32;
            if let Some(length) = content_length {
              if length == 0 {
                if let Some(msg) = message_type {
                  if msg == 0 {
                    return Ok(Message::Ping);
                  }
                  return Err(Error::Incomplete);
                }
              }
              let end_pos = (start_pos + length) as usize;
              let slice = &src.get_ref()[(start_pos as usize)..end_pos];
              let port = u16::from_radix_10(slice);
              // let text = String::from_utf8(slice.to_vec());
              if port.1 == 0 {
                return Err(Error::Incomplete);
              }
              return Ok(Message::Election { sender: port.0 });
            }
          }
        },
        _ => break,
      }
    }
    Ok(Message::Ping)
  }
}

fn get_u8 (src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
  if src.has_remaining() {
    return Ok(src.get_u8());
  }
  Err(Error::Incomplete)
}

fn parse_int(src: &mut Cursor<&[u8]>) -> Result<u32, Error> {
  let start_pos = src.position() as usize;
  while get_u8(src)? != b'*' {
    src.advance(1)
  }
  let end_pos = src.position() as usize;
  return Ok(u32::from_radix_10(&src.get_ref()[start_pos..end_pos]).0);
}