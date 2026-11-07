use super::{Easings, RotationEasing, ScaleEasing, TranslationEasing};
use crate::{GetTRS, SetTRS};

impl<'trs, 's: 'trs> GetTRS<'s, &'trs TranslationEasing, &'trs RotationEasing, &'trs ScaleEasing> for Easings {
    fn get_translation(&self) -> &TranslationEasing {
        &self.translation
    }

    fn get_rotation(&self) -> &RotationEasing {
        &self.rotation
    }

    fn get_scale(&self) -> &ScaleEasing {
        &self.scale
    }
}

impl SetTRS<TranslationEasing, RotationEasing, ScaleEasing> for Easings {
    fn set_translation(mut self, translation: impl Into<TranslationEasing>) -> Self {
        self.translation = translation.into();
        self
    }

    fn set_rotation(mut self, rotation: impl Into<RotationEasing>) -> Self {
        self.rotation = rotation.into();
        self
    }

    fn set_scale(mut self, scale: impl Into<ScaleEasing>) -> Self {
        self.scale = scale.into();
        self
    }
}
