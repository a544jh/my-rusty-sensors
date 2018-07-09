pub mod sqlite;

use chrono::prelude::*;

// enums as per https://www.mysensors.org/download/serial_api_20

pub struct Node {
    pub id: u32,
    pub name: String,
    pub version: String,
}

pub struct Sensor {
    pub id: u32,
    pub node_id: u32,
    pub sensor_type: Option<u32>,
    pub description: String,
}

#[derive(Debug)]
pub struct Reading {
    pub node_id: u32,
    pub sensor_id: u32,
    pub timestamp: DateTime<Local>,
    pub value: String,
    pub kind: u32,
}

pub trait Persist {
    fn store_node(&self, &Node);
    fn store_sensor(&self, &Sensor);
    fn store_reading(&self, &Reading);
}
