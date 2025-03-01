use crate::registry::PotionEffect;

#[packet(0x1D, server)]
pub struct EntityEffect {
    #[encoding("varint")]
    pub entity_id: i32,
    pub effect: PotionEffect,
    pub amplifier: i8,
    #[encoding("varint")]
    pub duration_in_seconds: i32,
    pub hide_particles: bool,
}
