use super::message;
use super::message::Sensor::*;
// use num::*;

pub fn encode(msg: &message::Message) -> String {
    //let i :u32 = *msg as u32;
    let command = &msg.command;
    let (i, _) = command.encode();
    i.to_string()
}

fn decode(msg_str: &str) -> message::Message {
    let i: u32 = msg_str.parse().unwrap();
    message::Message {
        node_id: 0,
        child_sensor_id: 0,
        command: message::Command::decode((i, 0)),
        ack: false,
        payload: message::PayloadType::Int(0),
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
        assert_eq!(msg_str, "0");
    }

    #[test]
    fn can_decode() {
        let msg_str = "0";
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
}
