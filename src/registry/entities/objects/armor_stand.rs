use gami_macros::entity;

#[entity(Object)]
pub struct ArmorStand {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    // Armor Stand
    pub armor_stand_flags: u8,
    pub head_rotation: (f32, f32, f32),      // pitch, yaw, roll
    pub body_rotation: (f32, f32, f32),      // pitch, yaw, roll
    pub left_arm_rotation: (f32, f32, f32),  // pitch, yaw, roll
    pub right_arm_rotation: (f32, f32, f32), // pitch, yaw, roll
    pub left_leg_rotation: (f32, f32, f32),  // pitch, yaw, roll
    pub right_leg_rotation: (f32, f32, f32), // pitch, yaw, roll
}

impl ArmorStand {
    fn update(&mut self, metadatas: &[Metadata]) {
        for metadata in metadatas {
            match metadata {
                Metadata::Byte(10, value) => self.armor_stand_flags = *value,
                Metadata::Pyr(11, p, y, r) => self.head_rotation = (*p, *y, *r),
                Metadata::Pyr(12, p, y, r) => self.body_rotation = (*p, *y, *r),
                Metadata::Pyr(13, p, y, r) => self.left_arm_rotation = (*p, *y, *r),
                Metadata::Pyr(14, p, y, r) => self.right_arm_rotation = (*p, *y, *r),
                Metadata::Pyr(15, p, y, r) => self.left_leg_rotation = (*p, *y, *r),
                Metadata::Pyr(16, p, y, r) => self.right_leg_rotation = (*p, *y, *r),
                _ => (),
            }
        }
    }
}
