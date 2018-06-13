use super::message;
use super::message::Sensor::*;
use super::message::Payload::*;
// use num::*;

pub fn encode(msg: &message::Message) -> String {
    let command = &msg.command;
    let (cmd, typ) = command.encode();
    let ack = match msg.ack {
        true => "1",
        false => "0",
    };
    format!("{};{};{};{};{};{}\n",
        msg.node_id,
        msg.child_sensor_id,
        cmd,
        ack,
        typ,
        msg.payload,
    )
}

fn decode(msg_str: &str) -> message::Message {
    let mut it = msg_str.split(";");
    let node_id: u32 = it.next().unwrap().parse().unwrap();
    let child_sensor_id: u32 = it.next().unwrap().parse().unwrap();
    let cmd: u32 = it.next().unwrap().parse().unwrap();
    let ack: u32 = it.next().unwrap().parse().unwrap();
    let typ: u32 = it.next().unwrap().parse().unwrap();
    let pl = it.next().unwrap().trim();

    message::Message {
        node_id,
        child_sensor_id,
        command: message::Command::decode((cmd, typ)),
        ack: if ack != 0 {true} else {false},
        payload: message::PayloadType::decode(pl),
    }
}

#[cfg(test)]
mod tests {
    use super::message;
    use super::*;

    #[test]
    fn can_encode() {
        let msg = message::Message {
            node_id: 0,
            child_sensor_id: 0,
            command: message::Command::Presentation(Door),
            ack: false,
            payload: message::PayloadType::Int(0),
        };
        let msg_str = encode(&msg);
        assert_eq!(msg_str, "0;0;0;0;0;0\n");
    }

    #[test]
    fn can_encode_last_sensor() {
        let msg = message::Message {
            node_id: 2,
            child_sensor_id: 3,
            command: message::Command::Presentation(WaterQuality),
            ack: false,
            payload: message::PayloadType::Int(5),
        };
        let msg_str = encode(&msg);
        assert_eq!(msg_str, "2;3;0;0;39;5\n");
    }

    #[test]
    fn can_encode_last_payload() {
        let msg = message::Message {
            node_id: 4,
            child_sensor_id: 2,
            command: message::Command::Set(PowerFactor),
            ack: true,
            payload: message::PayloadType::Float(2.5),
        };
        let msg_str = encode(&msg);
        assert_eq!(msg_str, "4;2;1;1;56;2.5\n");
    }

    #[test]
    fn can_decode() {
        let msg_str = "0;0;0;0;0;0\n";
        let msg = decode(&msg_str);
        let expected = message::Message {
            node_id: 0,
            child_sensor_id: 0,
            command: message::Command::Presentation(Door),
            ack: false,
            payload: message::PayloadType::Int(0),
        };
        assert_eq!(msg, expected);
    }

    #[test]
    fn can_decode_last_sensor() {
        let msg_str = "2;3;0;0;39;5\n";
        let msg = decode(&msg_str);
        let expected = message::Message {
            node_id: 2,
            child_sensor_id: 3,
            command: message::Command::Presentation(WaterQuality),
            ack: false,
            payload: message::PayloadType::Int(5),
        };
        assert_eq!(msg, expected);
    }

    #[test]
    fn can_decode_last_payload() {
        let msg_str = "4;2;1;1;56;2.5\n";
        let msg = decode(&msg_str);
        let expected = message::Message {
            node_id: 4,
            child_sensor_id: 2,
            command: message::Command::Set(PowerFactor),
            ack: true,
            payload: message::PayloadType::Float(2.5),
        };
        assert_eq!(msg, expected);
    }
}
