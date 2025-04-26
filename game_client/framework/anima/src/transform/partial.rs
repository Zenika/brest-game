use super::{Rotation, Scale, Transform, Translation};

mod trs;

pub struct PartialTransform {
    pub translation: Option<Translation>,
    pub rotation: Option<Rotation>,
    pub scale: Option<Scale>,
}

impl<T: Into<Option<Translation>>, R: Into<Option<Rotation>>, S: Into<Option<Scale>>>
    From<(T, R, S)> for PartialTransform
{
    fn from((translation, rotation, scale): (T, R, S)) -> Self {
        Self {
            translation: translation.into(),
            rotation: rotation.into(),
            scale: scale.into(),
        }
    }
}

impl From<Transform> for PartialTransform {
    fn from(transform: Transform) -> Self {
        Self {
            translation: transform.translation.into(),
            rotation: transform.rotation.into(),
            scale: transform.scale.into(),
        }
    }
}
