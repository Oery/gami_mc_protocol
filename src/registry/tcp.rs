use std::fmt::Debug;
use std::io::{Error, ErrorKind, Read, Result, Write};

use crate::serialization::{deserialize_varint, serialize_varint, Deserialize, Serialize};

pub enum Origins {
    Client,
    Server,
}

#[derive(Eq, PartialEq, Debug)]
pub enum States {
    Handshake,
    Status,
    Login,
    Play,
}

impl States {
    pub fn from_module_path(module: &str) -> States {
        match module {
            _ if module.contains("handshake") => States::Handshake,
            _ if module.contains("status") => States::Status,
            _ if module.contains("login") => States::Login,
            _ if module.contains("play") => States::Play,
            _ => panic!("A Packet must be defined in either a state module"),
        }
    }
}

impl Serialize for States {
    fn serialize(&self, buf: &mut dyn Write) -> Result<()> {
        match self {
            States::Handshake => serialize_varint(&0, buf),
            States::Status => serialize_varint(&1, buf),
            States::Login => serialize_varint(&2, buf),
            States::Play => serialize_varint(&3, buf),
        }
    }
}

impl Deserialize for States {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        match deserialize_varint(reader)? {
            0 => Ok(States::Handshake),
            1 => Ok(States::Status),
            2 => Ok(States::Login),
            3 => Ok(States::Play),
            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid state")),
        }
    }
}
