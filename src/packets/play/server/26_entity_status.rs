use gami_macros::packet;

#[packet(0x1A, server)]
pub struct EntityStatus {
    pub entity_id: i32,
    pub entity_status: i8,
}
