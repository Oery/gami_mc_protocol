use gami_macros::packet;

#[packet(0x03, client)]
pub struct Flying {
    pub on_ground: bool,
}
