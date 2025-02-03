use std::fmt::Debug;
use std::io::{Error, ErrorKind, Result};

use crate::registry::tcp::{Origins, States};
use crate::serialization::{Deserialize, Serialize, ToVarInt};

pub mod handshake;
pub mod login;
pub mod play;
pub mod status;

mod packets_enum;
pub use packets_enum::Packets;

pub trait Packet: Serialize + Deserialize + Debug + Send + Sync {
    fn serialize(&self, compression: i32) -> Result<Vec<u8>> {
        let mut data = Vec::new();

        if compression != -1 {
            data.push(0x00); // TODO: Implement compression
        }

        data.push(self.get_id());
        Serialize::serialize(self, &mut data)?;

        let mut bytes = (data.len() as i32).to_varint()?;
        bytes.extend(data);

        Ok(bytes)
    }

    fn deserialize(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        let mut reader = std::io::Cursor::new(bytes);
        Deserialize::deserialize(&mut reader)
    }

    fn get_origin(&self) -> Origins;
    fn get_name(&self) -> &'static str;
    fn get_state(&self) -> States;
    fn get_id(&self) -> u8;
}

#[derive(Debug)]
pub struct GlobalPacket<'a> {
    pub packet: &'a Packets,
}

pub trait ServerPacket: Packet {}
pub trait ClientPacket: Packet {}
