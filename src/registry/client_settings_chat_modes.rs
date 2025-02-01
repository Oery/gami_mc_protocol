use std::io;

use byteorder::{ReadBytesExt, WriteBytesExt};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::serialization::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(u8)]
pub enum ChatModes {
    Full = 0,
    CommandsOnly = 1,
    Hidden = 2,
}

impl Deserialize for ChatModes {
    fn deserialize<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let byte = reader.read_u8()?;
        ChatModes::try_from(byte).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

impl Serialize for ChatModes {
    fn serialize(&self, buf: &mut dyn std::io::Write) -> std::io::Result<()> {
        let byte = u8::from(*self);
        buf.write_u8(byte)
    }
}
