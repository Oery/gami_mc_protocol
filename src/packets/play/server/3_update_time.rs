use gami_macros::packet;

#[packet(0x03, server)]
pub struct UpdateTime {
    pub age: i64,
    pub time: i64,
}
