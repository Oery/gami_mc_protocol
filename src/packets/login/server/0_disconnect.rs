use gami_macros::packet;

#[packet(0x00, server)]
pub struct Disconnect {
    pub reason: String,
}
