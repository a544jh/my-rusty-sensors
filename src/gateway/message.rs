use num::ToPrimitive;
use num::FromPrimitive;

// https://www.mysensors.org/download/serial_api_20

#[derive(Debug, PartialEq)]
pub struct Message {
    pub node_id: u32,
    pub child_sensor_id: u32,
    pub command: Command,
    pub ack: bool,
    pub payload: PayloadType,
}

#[derive(Debug, PartialEq)]
pub enum PayloadType {
    Int(u32),
    Float(f32),
    Str(String),
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Presentation(Sensor),
    Set(Payload),
    Req(Payload),
    Internal(Internal),
    Stream,
}

impl Command {
    pub fn decode(ints: (u32, u32)) -> Command {
        let (cmd, cmd_type) = ints;

        match cmd {
            0 => {
                let typ = FromPrimitive::from_u32(cmd_type).unwrap();
                Command::Presentation(typ)
            }
            1 => {
                let typ = FromPrimitive::from_u32(cmd_type).unwrap();
                Command::Set(typ)
            }
            2 => {
                let typ = FromPrimitive::from_u32(cmd_type).unwrap();
                Command::Req(typ)
            }
            3 => {
                let typ = FromPrimitive::from_u32(cmd_type).unwrap();
                Command::Internal(typ)
            }
            4 => Command::Stream,
            _ => panic!("Invalid message type")
        }

        // Command::Presentation(SensorType::Door)
    }

    pub fn encode(&self) -> (u32, u32) {
        match self {
            Command::Presentation(typ) => {
                let i = typ.to_u32().unwrap();
                (0,i)
            }
            Command::Set(typ) => {
                let i = typ.to_u32().unwrap();
                (1,i)
            }
            Command::Req(typ) => {
                let i = typ.to_u32().unwrap();
                (2,i)
            }
            Command::Internal(typ) => {
                let i = typ.to_u32().unwrap();
                (3,i)
            }
            Command::Stream => (4,0)
        }
    }
}

#[derive(Debug, PartialEq, ToPrimitive, FromPrimitive)]
pub enum Sensor {
    Door,
    Motion,
    Smoke,
    Binary,
}

#[derive(Debug, PartialEq, ToPrimitive, FromPrimitive)]
pub enum Payload {
    Temperature,
    Humidity,
    Status,
    Percentage,
}

#[derive(Debug, PartialEq, ToPrimitive, FromPrimitive)]
pub enum Internal {
    BatteryLevel,
    Time,
    Version,
    IdRequest,
}
