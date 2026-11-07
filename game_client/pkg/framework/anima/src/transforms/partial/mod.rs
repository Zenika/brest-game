use super::{RotationTransform, ScaleTransform, Transforms, TranslationTransform};

mod trs;

pub struct PartialTransforms {
    pub translation: Option<TranslationTransform>,
    pub rotation: Option<RotationTransform>,
    pub scale: Option<ScaleTransform>,
}

impl<T: Into<Option<TranslationTransform>>, R: Into<Option<RotationTransform>>, S: Into<Option<ScaleTransform>>>
    From<(T, R, S)> for PartialTransforms
{
    fn from((translation, rotation, scale): (T, R, S)) -> Self {
        Self { translation: translation.into(), rotation: rotation.into(), scale: scale.into() }
    }
}

impl From<Transforms> for PartialTransforms {
    fn from(transforms: Transforms) -> Self {
        Self {
            translation: transforms.translation.into(),
            rotation: transforms.rotation.into(),
            scale: transforms.scale.into(),
        }
    }
}
