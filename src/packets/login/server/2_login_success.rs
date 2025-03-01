use gami_macros::packet;

#[packet(0x02, server)]
pub struct LoginSuccess {
    pub uuid: String,
    pub username: String,
}
