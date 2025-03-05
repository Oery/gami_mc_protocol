use gami_macros::packet;

#[packet(0x00, client)]
pub struct SetProtocol {
    #[encoding("varint")]
    pub protocol_version: i32,
    pub server_host: String,
    pub server_port: u16,
    pub next_state: States,
}
