use super::message;

pub fn encode(msg: &message::MessageType) -> String {
  let i :u32 = *msg as u32;
  i.to_string()
}