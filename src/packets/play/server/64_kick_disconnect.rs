use gami_macros::packet;

#[packet(0x40, server)]
pub struct KickDisconnect {
    pub reason: String,
}
