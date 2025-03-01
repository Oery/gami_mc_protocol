use gami_macros::packet;

#[packet(0x1B, server)]
pub struct AttachEntity {
    pub entity_id: i32,
    pub vehicle_id: i32,
    pub leash: bool,
}
