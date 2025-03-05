use gami_macros::packet;

#[packet(0x3C, server)]
pub struct ScoreboardUpdate {
    pub player: String,
    pub action: u8,
    pub objective: String,
    #[condition(self.action == 0)]
    #[encoding("varint")]
    pub value: Option<i32>,
}
