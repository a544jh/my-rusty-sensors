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
    lastReading: Reading,
}

pub struct Reading {
    timestamp: time::Instant,
    value: PayloadType,
}

impl Controller {
    pub fn handle_message(&mut self, message: &Message) {
        match message.command {
            Command::Set(pl) => {
                let reading = Reading {
                    timestamp: time::Instant::now(),
                    value: message.payload.clone(),
                };
                self.update_sensor(message.node_id, message.child_sensor_id, reading);
            }
            _ => (),
        }
    }

    fn get_node(&mut self, node_id: u32) -> &mut Node {
        if let Some(nod) = self.nodes.iter_mut().find(|n| n.id == node_id) {
            return nod;
        }
        let new_node = Node {
            id: node_id,
            name: "".to_string(),
            version: "".to_string(),
            sensors: Vec::new(),
        };
        self.nodes.push(new_node);
        self.nodes.iter_mut().find(|n| n.id == node_id).unwrap()
    }

    fn update_sensor(&mut self, node_id: u32, child_id: u32, reading: Reading) {
        let node = self.get_node(node_id);

        match node.sensors.iter_mut().find(|s| s.id == child_id) {
            Some(mut s) => {
                s.lastReading = reading;
            }
            None => {
                let new_sensor = Sensor {
                    id: child_id,
                    sensorType: SensorType::Door, //TODO, need to get from presentation msg...
                    lastReading: reading,
                };
                node.sensors.push(new_sensor);
            }
        };
    }
}
