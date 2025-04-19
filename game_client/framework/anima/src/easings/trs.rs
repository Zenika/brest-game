use crate::{GetTRS, GetTRSMut};

use super::{Easings, RotationEasing, ScaleEasing, TranslationEasing};

impl GetTRS<TranslationEasing, RotationEasing, ScaleEasing> for Easings {
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

impl GetTRSMut<TranslationEasing, RotationEasing, ScaleEasing> for Easings {
    fn get_translation_mut(&mut self) -> &mut TranslationEasing {
        &mut self.translation
    }

    fn get_rotation_mut(&mut self) -> &mut RotationEasing {
        &mut self.rotation
    }

    fn get_scale_mut(&mut self) -> &mut ScaleEasing {
        &mut self.scale
    }
}
