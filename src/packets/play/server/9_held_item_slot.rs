use gami_macros::packet;

#[packet(0x09, server)]
pub struct HeldItemSlot {
    pub slot: i8,
}
