use gami_macros::packet;

#[packet(0x09, client)]
pub struct HeldItemSlot {
    pub slot: i16,
}
