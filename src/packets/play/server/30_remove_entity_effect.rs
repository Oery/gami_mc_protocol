use crate::packets::Packet;
use crate::registry::PotionEffects;
use crate::serialization::{deserialize_varint, serialize_varint};
use gami_macros::{packet, Deserialize, Serialize};

#[packet(0x1E, server)]
pub struct RemoveEntityEffect {
    #[encoding("varint")]
    pub entity_id: i32,
    pub effect: PotionEffects,
}
