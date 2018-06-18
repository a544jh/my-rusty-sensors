pub mod message;
pub mod serial;

pub trait Gateway {
    fn receive(&mut self) -> Result<message::Message, serial::MalformedStringError>;
    fn send(&mut self, &message::Message); //->Result?
}
