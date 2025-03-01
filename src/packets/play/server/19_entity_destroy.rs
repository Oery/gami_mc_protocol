use gami_macros::packet;

#[packet(0x13, server)]
pub struct EntityDestroy {
    #[encoding("varint")]
    pub entity_ids: Vec<i32>,
}
