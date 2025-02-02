use crate::packets::Packet;
use crate::registry::PotionEffects;
use crate::serialization::{deserialize_varint, serialize_varint};
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x1D, server)]
pub struct EntityEffect {
    #[encoding("varint")]
    pub entity_id: i32,
    pub effect: PotionEffects,
    pub amplifier: i8,
    #[encoding("varint")]
    pub duration_in_seconds: i32,
    pub hide_particles: bool,
}
