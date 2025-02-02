use crate::packets::Packet;
use crate::serialization::{deserialize_varint_vec, serialize_varint_vec};
use macros::{packet, Deserialize, Serialize};

#[packet(0x13, server)]
pub struct EntityDestroy {
    #[encoding("varint")]
    pub entity_ids: Vec<i32>,
}
