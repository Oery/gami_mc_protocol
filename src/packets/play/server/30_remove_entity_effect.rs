use gami_macros::packet;

use crate::registry::PotionEffect;

#[packet(0x1E, server)]
pub struct RemoveEntityEffect {
    #[encoding("varint")]
    pub entity_id: i32,
    pub effect: PotionEffect,
}
