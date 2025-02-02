use crate::packets::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x00, client)]
pub struct KeepAlive {
    #[encoding("varint")]
    pub id: i32,
}
