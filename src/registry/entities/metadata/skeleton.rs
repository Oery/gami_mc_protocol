#[derive(Debug, Default, PartialEq)]
pub enum SkeletonKind {
    #[default]
    Skeleton = 0,
    Wither = 1,
}

impl From<i8> for SkeletonKind {
    fn from(value: i8) -> Self {
        match value {
            0 => SkeletonKind::Skeleton,
            1 => SkeletonKind::Wither,
            _ => SkeletonKind::Skeleton,
        }
    }
}
