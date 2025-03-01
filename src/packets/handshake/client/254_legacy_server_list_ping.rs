use gami_macros::packet;

#[packet(0xfe, client)]
pub struct LegacyServerListPing {
    pub payload: u8,
}
