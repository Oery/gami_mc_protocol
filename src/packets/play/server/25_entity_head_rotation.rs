use gami_macros::packet;

#[packet(0x19, server)]
pub struct EntityHeadRotation {
    #[encoding("varint")]
    pub entity_id: i32,
    pub head_yaw: i8,
}
