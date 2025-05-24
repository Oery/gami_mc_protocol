use gami_macros::entity;

use crate::registry::entities::metadata::SkeletonKind;

#[entity(LivingEntity)]
pub struct Skeleton {
    pub id: i32,
    pub flags: u8,
    pub air: i16,
    pub name_tag: String,
    pub always_show_name_tag: bool,
    pub is_silent: bool,
    pub ai_disabled: bool,
    pub kind: SkeletonKind,
}

impl Skeleton {
    fn update(&mut self, metadatas: &[Metadata]) {
        for metadata in metadatas {
            if let Metadata::Byte(13, value) = metadata {
                self.kind = SkeletonKind::from(*value as i8)
            }
        }
    }
}
