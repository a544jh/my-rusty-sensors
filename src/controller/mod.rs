use super::gateway::message::Command;
use super::gateway::message::Message;
use super::gateway::message::PayloadType;
use super::gateway::message::Sensor as SensorType;
use std::time;

pub struct Controller {
    nodes: Vec<Node>,
}

pub struct Node {
    id: u32,
    name: String,
    version: String,
    sensors: Vec<Sensor>,
}

pub struct Sensor {
    id: u32,
    sensorType: SensorType,
    lastReading: Option<Reading>,
}

#[derive(Debug)]
pub struct Reading {
    timestamp: time::Instant,
    value: PayloadType,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            nodes: Vec::new()
        }
    }

    pub fn handle_message(&mut self, message: &Message) {
        match message.command {
            Command::Set(_pl) => {
                let reading = Reading {
                    timestamp: time::Instant::now(),
                    value: message.payload.clone(),
                };
                self.update_sensor(message.node_id, message.child_sensor_id, reading);
            }
            _ => (),
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

    fn update_sensor(&mut self, node_id: u32, child_id: u32, reading: Reading) {
        match self.find_sensor(node_id, child_id) {
            Some(s) => {
                s.lastReading = Some(reading);
                return;
            }
            None => (),
        };
        let new_sensor = Sensor {
            id: child_id,
            sensorType: SensorType::Door, //TODO, need to get from presentation msg...
            lastReading: Some(reading),
        };

        match self.find_node(node_id) {
            Some(n) => {
                n.sensors.push(new_sensor);
                return;
            }
            None => (),
        }

        let new_node = Node {
            id: node_id,
            name: "".to_string(),
            version: "".to_string(),
            sensors: vec![new_sensor],
        };
        self.add_node(new_node);
    }

    pub fn print_status (&self) {
        for node in self.nodes.iter() {
            for sensor in node.sensors.iter() {
                if let Some(ref lr) = sensor.lastReading {
                    println!("{} {} {:?} {:?}", node.id, sensor.id, sensor.sensorType, lr.value)
                }
            }
        }
    }
}
