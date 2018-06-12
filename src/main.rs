pub mod gateway;

use gateway::serial;

fn main() {
    println!("Hello, world!");
    let msg = gateway::message::MessageType::Test1;
    let msg_str = serial::encode(&msg);
    println!("{:?}", msg);
    println!("{}", msg_str);
}
