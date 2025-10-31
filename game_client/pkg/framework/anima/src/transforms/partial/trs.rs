use super::PartialTransforms;
use crate::{GetTRS, RotationTransform, ScaleTransform, TranslationTransform};

impl<'trs, 's: 'trs>
    GetTRS<'s, &'trs Option<TranslationTransform>, &'trs Option<RotationTransform>, &'trs Option<ScaleTransform>>
    for PartialTransforms
{
    fn get_translation(&self) -> &Option<TranslationTransform> {
        &self.translation
    }

    fn get_rotation(&self) -> &Option<RotationTransform> {
        &self.rotation
    }

    fn get_scale(&self) -> &Option<ScaleTransform> {
        &self.scale
    }
}
