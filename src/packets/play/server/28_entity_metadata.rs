use crate::packets::Packet;
use crate::serialization::{
    deserialize_metadatas_vec, deserialize_varint, serialize_metadatas_vec, serialize_varint,
    Metadata,
};
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x1C, server)]
pub struct EntityMetadata {
    #[encoding("varint")]
    pub entity_id: i32,
    #[encoding("metadatas")]
    pub metadatas: Vec<Metadata>,
}
