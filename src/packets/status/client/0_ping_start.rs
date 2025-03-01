use gami_macros::packet;

#[packet(0x00, client)]
pub struct PingStart {}
