use super::gateway::message::Command;
use super::gateway::message::Internal;
use super::gateway::message::Kind;
use super::gateway::message::Message;
use super::gateway::message::PayloadType;
use super::gateway::message::Sensor as SensorType;
use super::gateway::Gateway;
use chrono::prelude::*;

pub struct Controller {
    gateway: Box<Gateway>,
    nodes: Vec<Node>,
    presentation_request_skips: u32,
}

pub struct Node {
    id: u32,
    name: String,
    version: String,
    sensors: Vec<Sensor>,
}

pub struct Sensor {
    id: u32,
    sensor_type: Option<SensorType>,
    description: String,
    last_reading: Option<Reading>,
}

#[derive(Debug)]
pub struct Reading {
    timestamp: DateTime<Local>,
    value: PayloadType,
    kind: Kind,
}

impl Controller {
    pub fn new(gateway: Box<Gateway>) -> Controller {
        Controller {
            gateway,
            nodes: Vec::new(),
            presentation_request_skips: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            //clear terminal
            print!("{}[2J", 27 as char);
            print!("{}[0;0H", 27 as char);;
            let message = self.gateway.receive();
            match message {
                Ok(msg) => self.handle_message(&msg),
                Err(e) => println!("{}", e),
            }
            self.request_unknown_sensors();
            self.print_status();
            //println!("{}", buf.trim()); //TODO: maybe implement Gateway::rawMessage or smth...
        }
    }

    fn handle_message(&mut self, message: &Message) {
        match message.command {
            Command::Set(kind) => {
                let reading = Reading {
                    timestamp: Local::now(),
                    value: message.payload.clone(),
                    kind,
                };
                self.update_sensor(message.node_id, message.child_sensor_id, |s| {
                    s.last_reading = Some(reading)
                });
            }
            Command::Presentation(typ) => {
                let desc = message.payload.get_str();
                self.update_sensor(message.node_id, message.child_sensor_id, |s| {
                    s.sensor_type = Some(typ);
                    s.description = desc;
                })
            }
            Command::Internal(internal) => match internal {
                Internal::SketchName => {
                    let name = message.payload.get_str();
                    self.update_node(message.node_id, |n| n.name = name)
                }
                Internal::SketchVersion => {
                    let version = message.payload.get_str();
                    self.update_node(message.node_id, |n| n.version = version)
                }
                _ => (),
            },
            _ => (),
        }
    }

    fn request_unknown_sensors(&mut self) {
        for node in self.nodes.iter() {
            for sensor in node.sensors.iter() {
                if let None = sensor.sensor_type {
                    let msg = Message {
                        node_id: node.id,
                        child_sensor_id: sensor.id,
                        command: Command::Internal(Internal::Presentation),
                        ack: true,
                        payload: PayloadType::Str(String::new()),
                    };
                    // don't spam that poor arduino... (TODO: this properly (with some sensor state maybe...))
                    // the replies will eventually arrive
                    if self.presentation_request_skips > 0 {
                        self.presentation_request_skips -= 1;
                        return;
                    }
                    self.presentation_request_skips = 20;
                    self.gateway.send(&msg);
                }
            }
        }
    }

    fn find_node(&mut self, node_id: u32) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.id == node_id)
    }

    fn find_sensor(&mut self, node_id: u32, child_id: u32) -> Option<&mut Sensor> {
        match self.find_node(node_id) {
            None => None,
            Some(n) => n.sensors.iter_mut().find(|s| s.id == child_id),
        }
    }

    fn add_node(&mut self, node: Node) -> &mut Node {
        let id = node.id;
        self.nodes.push(node);
        self.find_node(id).unwrap()
    }

    fn update_node<F>(&mut self, node_id: u32, func: F)
    where
        F: FnOnce(&mut Node),
    {
        match self.find_node(node_id) {
            Some(n) => {
                func(n);
                return;
            }
            None => (),
        }
        let new_node = Node {
            id: node_id,
            name: String::new(),
            version: String::new(),
            sensors: Vec::new(),
        };
        self.add_node(new_node);
    }

    fn update_sensor<F>(&mut self, node_id: u32, child_id: u32, func: F)
    where
        F: FnOnce(&mut Sensor),
    {
        match self.find_sensor(node_id, child_id) {
            Some(s) => {
                func(s);
                return;
            }
            None => (),
        };
        let mut new_sensor = Sensor {
            id: child_id,
            sensor_type: None,
            description: String::new(),
            last_reading: None,
        };
        func(&mut new_sensor);
        match self.find_node(node_id) {
            Some(n) => {
                n.sensors.push(new_sensor);
                return;
            }
            None => (),
        }

        let new_node = Node {
            id: node_id,
            name: String::new(),
            version: String::new(),
            sensors: vec![new_sensor],
        };
        self.add_node(new_node);
    }

    pub fn print_status(&self) {
        for node in self.nodes.iter() {
            println!("{} {} {}", node.id, node.name, node.version);
            for sensor in node.sensors.iter() {
                if let Some(ref lr) = sensor.last_reading {
                    let typ = match sensor.sensor_type {
                        None => String::from("Unknown"),
                        Some(t) => format!("{:?}", t),
                    };
                    println!(
                        "  {} {} {} {:?} {} {}",
                        sensor.id,
                        sensor.description,
                        typ,
                        lr.kind,
                        lr.value,
                        lr.timestamp.format("%H:%M")
                    )
                }
            }
        }
    }
}
