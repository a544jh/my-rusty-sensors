extern crate num;
#[macro_use]
extern crate num_derive;

pub mod gateway;

use gateway::message;
use gateway::message::SensorType::*;
use gateway::serial;

fn main() {
    println!("Hello, world!");
    let msg = message::Message {
        node_id: 0,
        child_sensor_id: 0,
        command: message::Command::Presentation(Door),
        ack: false,
        payload: message::Payload::Int(0),
    };
    let msg_str = serial::encode(&msg);
    println!("{:?}", msg);
    println!("{}", msg_str);
}
