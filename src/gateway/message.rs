use num::ToPrimitive;
use num::FromPrimitive;

// https://www.mysensors.org/download/serial_api_20

#[derive(Debug, PartialEq)]
pub struct Message {
    pub node_id: u32,
    pub child_sensor_id: u32,
    pub command: Command,
    pub ack: bool,
    pub payload: Payload,
}

#[derive(Debug, PartialEq)]
pub enum Payload {
    Int(u32),
    Float(f32),
    Str(String),
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Presentation(SensorType),
    Set(PayloadType),
    Req(PayloadType),
    Internal(InternalType),
    Stream,
}

impl Command {
    pub fn from(ints: (u32, u32)) -> Command {
        let (cmd, cmd_type) = ints;

        match cmd {
            0 => {
                let sens = SensorType::from_u32(cmd_type).unwrap();
                Command::Presentation(sens)
            }
            1 => {
                let payl = PayloadType::from_u32(cmd_type).unwrap();
                Command::Set(payl)
            }
            _ => panic!("Invalid message type")
        }

        // Command::Presentation(SensorType::Door)
    }

    pub fn to(&self) -> (u32, u32) {
        match self {
            Command::Presentation(sens) => {
                let i = sens.to_u32().unwrap();
                (0,i)
            }
            Command::Set(_) => (1,1),
            _ => panic!("Not yet implemented")
        }
    }
}

#[derive(Debug, PartialEq, ToPrimitive, FromPrimitive)]
pub enum SensorType {
    Door,
    Motion,
    Smoke,
    Binary,
}

#[derive(Debug, PartialEq, ToPrimitive, FromPrimitive)]
pub enum PayloadType {
    Temperature,
    Humidity,
    Status,
    Percentage,
}

#[derive(Debug, PartialEq, ToPrimitive, FromPrimitive)]
pub enum InternalType {
    BatteryLevel,
    Time,
    Version,
    IdRequest,
}
