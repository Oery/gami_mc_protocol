use gami_macros::packet;

#[packet(0x14, server)]
pub struct Entity {
    #[encoding("varint")]
    pub entity_id: i32,
}
