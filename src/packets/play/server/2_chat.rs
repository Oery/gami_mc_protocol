use gami_macros::packet;

#[packet(0x02, server)]
pub struct Chat {
    pub message: String,
    pub position: ChatPosition,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[encoding("u8")]
pub enum ChatPosition {
    Chat = 1,
    Hotbar = 2,
    System = 4,
}
