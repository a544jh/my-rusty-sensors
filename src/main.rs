extern crate num;
#[macro_use]
extern crate num_derive;
extern crate serialport;

use std::io::BufRead;
use std::io::BufReader;
use std::time::Duration;

pub mod controller;
pub mod gateway;

use gateway::serial;

fn main() {
    println!("Hello, world!");
    let mut port = serialport::open("/dev/ttyUSB0").unwrap();
    let _ = port.set_baud_rate(serialport::BaudRate::Baud115200);
    let _ = port.set_timeout(Duration::from_secs(200));
    let mut reader = BufReader::new(port);
    loop {
        let mut buf = String::new();
        let _res = reader.read_line(&mut buf);
        println!("{}", buf.trim());
        let message = serial::decode(&buf);
        println!("{:#?}", message);
    }
}
