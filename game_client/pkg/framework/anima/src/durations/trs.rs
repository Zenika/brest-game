use crate::{GetTRS, GetTRSMut};

use super::{Durations, RotationDuration, ScaleDuration, TranslationDuration};

impl GetTRS<TranslationDuration, RotationDuration, ScaleDuration> for Durations {
    fn get_translation(&self) -> &TranslationDuration {
        &self.translation
    }

    fn get_rotation(&self) -> &RotationDuration {
        &self.rotation
    }

    fn get_scale(&self) -> &ScaleDuration {
        &self.scale
    }
}

impl GetTRSMut<TranslationDuration, RotationDuration, ScaleDuration> for Durations {
    fn get_translation_mut(&mut self) -> &mut TranslationDuration {
        &mut self.translation
    }

    fn get_rotation_mut(&mut self) -> &mut RotationDuration {
        &mut self.rotation
    }

    fn get_scale_mut(&mut self) -> &mut ScaleDuration {
        &mut self.scale
    }
}
