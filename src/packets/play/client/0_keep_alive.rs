use gami_macros::packet;

#[packet(0x00, client)]
pub struct KeepAlive {
    #[encoding("varint")]
    pub id: i32,
}
