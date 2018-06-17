extern crate num;
#[macro_use]
extern crate num_derive;
extern crate serialport;

use std::io::BufRead;
use std::io::BufReader;
use std::time::Duration;

pub mod controller;
pub mod gateway;

use controller::Controller;
use gateway::serial;

fn main() {
    println!("Hello, world!");
    let mut port = serialport::open("/dev/ttyUSB0").unwrap();
    let _ = port.set_baud_rate(serialport::BaudRate::Baud115200);
    let _ = port.set_timeout(Duration::from_secs(200));
    let mut reader = BufReader::new(port);

    let mut controller = Controller::new();

    loop {
        print!("{}[2J", 27 as char);
        print!("{}[0;0H", 27 as char);
        let mut buf = String::new();
        let _res = reader.read_line(&mut buf);
        let message = serial::decode(&buf);
        //println!("{:#?}", message);
        if let Ok(msg) = message {
            controller.handle_message(&msg)
        }
        controller.print_status();
        println!("{}", buf.trim());
    }
}
