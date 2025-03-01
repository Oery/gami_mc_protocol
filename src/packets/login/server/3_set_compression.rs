use gami_macros::packet;

#[packet(0x03, server)]
pub struct SetCompression {
    #[encoding("varint")]
    pub threshold: i32,
}
