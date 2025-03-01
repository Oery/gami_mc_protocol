use gami_macros::packet;

#[packet(0x00, server)]
pub struct KeepAlive {
    #[encoding("varint")]
    pub id: i32,
}
