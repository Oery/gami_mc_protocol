use std::io::{Error, ErrorKind, Read, Result, Write};

use atomic_enum::atomic_enum;
use derive_more::Display;

use crate::serialization::encoding::varint::{deserialize_varint_i32, serialize_varint_i32};
use crate::serialization::{Deserialize, Serialize};

/// Role of the sender
#[derive(Eq, PartialEq, Debug, Clone, Copy, Display)]
pub enum Origin {
    Client,
    Server,
}

impl Display for Origins {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Origins::Client => write!(f, "Client"),
            Origins::Server => write!(f, "Server"),
        }
    }
}
#[atomic_enum]
#[derive(Eq, PartialEq, Display)]
pub enum State {
    Handshake,
    Status,
    Login,
    Play,
}

impl State {
    pub fn from_module_path(module: &str) -> State {
        match module {
            _ if module.contains("handshake") => State::Handshake,
            _ if module.contains("status") => State::Status,
            _ if module.contains("login") => State::Login,
            _ if module.contains("play") => State::Play,
            _ => panic!("A Packet must be defined in either a state module"),
        }
    }
}

impl Serialize for State {
    fn serialize(&self, buf: &mut dyn Write) -> Result<()> {
        match self {
            State::Handshake => serialize_varint_i32(&0, buf),
            State::Status => serialize_varint_i32(&1, buf),
            State::Login => serialize_varint_i32(&2, buf),
            State::Play => serialize_varint_i32(&3, buf),
        }
    }
}

impl Deserialize for State {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        match deserialize_varint_i32(reader)? {
            0 => Ok(State::Handshake),
            1 => Ok(State::Status),
            2 => Ok(State::Login),
            3 => Ok(State::Play),
            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid state")),
        }
    }
}
