use num::FromPrimitive;
use std::fmt;

// https://www.mysensors.org/download/serial_api_20

#[derive(Debug, PartialEq)]
pub struct Message {
    pub node_id: u32,
    pub child_sensor_id: u32,
    pub command: Command,
    pub ack: bool,
    pub payload: PayloadType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PayloadType {
    Int(u32),
    Float(f32),
    Str(String),
}

impl PayloadType {
    pub fn get_str(&self) -> String {
        match self {
            PayloadType::Str(s) => s.clone(),
            _ => String::new(),
        }
    }
}

impl fmt::Display for PayloadType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PayloadType::Int(i) => write!(f, "{}", i),
            PayloadType::Float(fl) => write!(f, "{}", fl), //TODO: may need fxing
            PayloadType::Str(s) => write!(f, "{}", s),
        }
    }
}

impl PayloadType {
    pub fn decode(s: &str) -> PayloadType {
        if let Ok(u) = s.parse::<u32>() {
            return PayloadType::Int(u);
        }
        if let Ok(f) = s.parse::<f32>() {
            return PayloadType::Float(f);
        }
        PayloadType::Str(s.to_string())
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Presentation(Sensor),
    Set(Kind),
    Req(Kind),
    Internal(Internal),
    Stream,
}

impl Command {
    pub fn decode(ints: (u32, u32)) -> Option<Command> {
        let (cmd, typ) = ints;

        match cmd {
            0 => FromPrimitive::from_u32(typ).map(|t| Command::Presentation(t)),
            1 => FromPrimitive::from_u32(typ).map(|t| Command::Set(t)),
            2 => FromPrimitive::from_u32(typ).map(|t| Command::Req(t)),
            3 => FromPrimitive::from_u32(typ).map(|t| Command::Internal(t)),
            4 => Some(Command::Stream),
            _ => None,
        }
    }

    pub fn encode(&self) -> (u32, u32) {
        match self {
            Command::Presentation(typ) => {
                let i = *typ as u32;
                (0, i)
            }
            Command::Set(typ) => {
                let i = *typ as u32;
                (1, i)
            }
            Command::Req(typ) => {
                let i = *typ as u32;
                (2, i)
            }
            Command::Internal(typ) => {
                let i = *typ as u32;
                (3, i)
            }
            Command::Stream => (4, 0),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum Sensor {
    Door,
    Motion,
    Smoke,
    Binary,
    Dimmer,
    Cover,
    Temperature,
    Humidity,
    Barometer,
    Wind,
    Rain,
    UV,
    Weight,
    Power,
    Heater,
    Distance,
    LightLevel,
    ArduinoNode,
    ArduinoRepeaterNode,
    Lock,
    IR,
    Water,
    AirQuality,
    Custom,
    Dust,
    SceneController,
    RgbLight,
    RgbwLight,
    ColorSensor,
    HVAC,
    Multimeter,
    Sprinkler,
    WaterLeak,
    Sound,
    Vibration,
    Moisture,
    Info,
    Gas,
    GPS,
    WaterQuality,
}

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum Kind {
    Temperature,
    Humidity,
    Status,
    Percentage,
    Pressure,
    WeatherForecast,
    RainAmount,
    RainRate,
    WindSpeed,
    Gust,
    Direction,
    UV,
    Weight,
    Distance,
    Impedance,
    Armed,
    Tripped,
    Watt,
    KWH,
    SceneOn,
    SceneOff,
    HvacFlowState,
    HvacSpeed,
    LightLevel,
    Var1,
    Var2,
    Var3,
    Var4,
    Var5,
    Up,
    Down,
    Stop,
    IrSend,
    IrReceive,
    WaterFlow,
    WaterVolume,
    LockStatus,
    Level,
    Voltage,
    Current,
    RGB,
    RGBW,
    ID,
    UnitPrefix,
    HvacSetpointCool,
    HvacSetpointHeat,
    HvacFlowMode,
    Text,
    Custom,
    Position,
    IrRecord,
    WaterPh,
    WaterOrp,
    WaterEc,
    ReactivePower,
    ApparentPower,
    PowerFactor,
}

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum Internal {
    BatteryLevel,
    Time,
    Version,
    IdRequest,
    IdResponse,
    InclusionMode,
    Config,
    FindParent,
    FindParentResponse,
    LogMessage,
    Children,
    SketchName,
    SketchVersion,
    Reboot,
    GatewayReady,
    SigningPresentation,
    NonceRequest,
    NonceResponce,
    HeartbeatRequest,
    Presentation,
    DiscoverRequest,
    DiscoverResponse,
    HeartbeatResponse,
    Locked,
    Ping,
    Pong,
    RegistrationRequest,
    RegistrationResponse,
    Debug,
}
