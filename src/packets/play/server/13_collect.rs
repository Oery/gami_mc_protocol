use gami_macros::packet;

#[packet(0x0D, server)]
pub struct Collect {
    #[encoding("varint")]
    pub collected_entity_id: i32,
    #[encoding("varint")]
    pub collector_entity_id: i32,
}
