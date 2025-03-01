use gami_macros::packet;

#[packet(0x04, client)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub on_ground: bool,
}
