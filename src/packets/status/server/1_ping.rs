use gami_macros::packet;

#[packet(0x01, server)]
pub struct Ping {
    pub time: i64,
}
