use gami_macros::packet;
#[packet(0x1C, server)]
pub struct EntityMetadata {
    #[encoding("varint")]
    pub entity_id: i32,
    #[encoding("metadatas")]
    pub metadatas: Vec<Metadata>,
}
