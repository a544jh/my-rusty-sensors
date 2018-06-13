use super::message;
use num;

pub fn encode(msg: &message::MessageType) -> String {
  let i :u32 = *msg as u32;
  i.to_string()
}

fn decode(msg_str: &str) -> message::MessageType {
  let i :u32 = msg_str.parse().unwrap();
  num::FromPrimitive::from_u32(i).unwrap()
}

#[cfg(test)]
mod tests {
  use super::message;
  use super::*;

  #[test]
  fn can_encode() {
    let msg = message::MessageType::Test1;
    let msg_str = encode(&msg);
    assert_eq!(msg_str, "0");
  }

  #[test]
  fn can_decode() {
    let msg_str = "0";
    let msg = decode(&msg_str);
    assert_eq!(msg, message::MessageType::Test1);
  }
}