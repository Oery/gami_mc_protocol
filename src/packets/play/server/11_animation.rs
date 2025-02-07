use gami_macros::{packet, Deserialize, Serialize};

use crate::packets::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};

#[packet(0x0B, server)]
pub struct Animation {
    #[encoding("varint")]
    pub entity_id: i32,
    pub animation: AnimationKind,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
#[encoding("u8")]
pub enum AnimationKind {
    SwingArm = 0,
    TakeDamage = 1,
    LeaveBed = 2,
    Eat = 3,
    CriticalEffect = 4,
    MagicalCriticalEffect = 5,
}
