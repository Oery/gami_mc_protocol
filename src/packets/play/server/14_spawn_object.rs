use gami_macros::packet;

use crate::registry::entities::objects::ObjectKind;

#[packet(0x0E, server)]
pub struct SpawnObject {
    #[encoding("varint")]
    pub entity_id: i32,
    pub kind: ObjectKind,
    #[encoding("fixed_point_i32")]
    pub x: i32,
    #[encoding("fixed_point_i32")]
    pub y: i32,
    #[encoding("fixed_point_i32")]
    pub z: i32,
    pub yaw: u8,
    pub pitch: u8,
    pub data: i32,
    #[condition(self.data != 0)]
    pub velocity_x: Option<i16>,
    #[condition(self.data != 0)]
    pub velocity_y: Option<i16>,
    #[condition(self.data != 0)]
    pub velocity_z: Option<i16>,
}
