use crate::packets::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x03, server)]
pub struct SetCompression {
    #[encoding("varint")]
    pub threshold: i32,
}
