use super::{Durations, RotationDuration, ScaleDuration, TranslationDuration};
use crate::{GetTRS, SetTRS};

impl<'trs, 's: 'trs> GetTRS<'s, &'trs TranslationDuration, &'trs RotationDuration, &'trs ScaleDuration> for Durations {
    fn get_translation(&'s self) -> &'trs TranslationDuration {
        &self.translation
    }

    fn get_rotation(&'s self) -> &'trs RotationDuration {
        &self.rotation
    }

    fn get_scale(&'s self) -> &'trs ScaleDuration {
        &self.scale
    }
}

impl SetTRS<TranslationDuration, RotationDuration, ScaleDuration> for Durations {
    fn set_translation(mut self, translation: impl Into<TranslationDuration>) -> Self {
        self.translation = translation.into();
        self
    }

    fn set_rotation(mut self, rotation: impl Into<RotationDuration>) -> Self {
        self.rotation = rotation.into();
        self
    }

    fn set_scale(mut self, scale: impl Into<ScaleDuration>) -> Self {
        self.scale = scale.into();
        self
    }
}
