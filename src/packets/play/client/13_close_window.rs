use gami_macros::packet;

#[packet(0x0D, client)]
pub struct CloseWindow {
    pub window_id: u8,
}
