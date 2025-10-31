use super::{RotationTransform, ScaleTransform, TranslationTransform};
use crate::transforms::Transforms;
use crate::{GetTRS, SetTRS};

impl<'trs, 's: 'trs> GetTRS<'s, &'trs TranslationTransform, &'trs RotationTransform, &'trs ScaleTransform>
    for Transforms
{
    fn get_translation(&self) -> &TranslationTransform {
        &self.translation
    }

    fn get_rotation(&self) -> &RotationTransform {
        &self.rotation
    }

    fn get_scale(&self) -> &ScaleTransform {
        &self.scale
    }
}

impl SetTRS<TranslationTransform, RotationTransform, ScaleTransform> for Transforms {
    fn set_translation(mut self, translation: impl Into<TranslationTransform>) -> Self {
        self.translation = translation.into();
        self
    }

    fn set_rotation(mut self, rotation: impl Into<RotationTransform>) -> Self {
        self.rotation = rotation.into();
        self
    }

    fn set_scale(mut self, scale: impl Into<ScaleTransform>) -> Self {
        self.scale = scale.into();
        self
    }
}
