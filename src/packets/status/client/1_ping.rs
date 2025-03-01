use gami_macros::packet;

#[packet(0x01, client)]
pub struct Ping {
    pub time: i64,
}
