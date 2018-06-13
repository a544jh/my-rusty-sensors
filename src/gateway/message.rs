#[derive(Debug, ToPrimitive, FromPrimitive, PartialEq)]
pub enum MessageType {
    Test1 = 1,
    Test2,
}