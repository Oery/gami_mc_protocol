use std::io;

use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::serialization::{serialize_varint, VarIntReader};
use crate::serialization::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(i32)]
pub enum ClientCommandActions {
    PerformRespawn = 0,
    RequestStats = 1,
    TakingInventoryAchievement = 2,
}

impl Deserialize for ClientCommandActions {
    fn deserialize<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        ClientCommandActions::try_from(reader.read_varint()?)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

impl Serialize for ClientCommandActions {
    fn serialize(&self, buf: &mut dyn io::Write) -> io::Result<()> {
        let value = i32::from(*self);
        serialize_varint(&value, buf)
    }
}
