use crate::packets::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use macros::{packet, Deserialize, Serialize};

#[packet(0x06, server)]
pub struct UpdateHealth {
    pub health: f32,
    #[encoding("varint")]
    pub food: i32,
    pub food_saturation: f32,
}
