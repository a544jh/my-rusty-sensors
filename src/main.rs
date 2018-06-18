extern crate num;
#[macro_use]
extern crate num_derive;
extern crate bufstream;
extern crate chrono;
extern crate serialport;

use std::time::Duration;

pub mod controller;
pub mod gateway;

use controller::Controller;
use gateway::serial::SerialGateway;

fn main() {
    println!("Hello, world!");
    let mut port = serialport::open("/dev/ttyUSB0").unwrap();
    let _ = port.set_baud_rate(serialport::BaudRate::Baud115200);
    let _ = port.set_timeout(Duration::from_secs(200));

    let gateway = Box::new(SerialGateway::new(port));
    let mut controller = Controller::new(gateway);

    controller.run();
}
