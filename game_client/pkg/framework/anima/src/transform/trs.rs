use crate::{GetTRS, GetTRSMut};

use super::{Rotation, Scale, Transform, Translation};

impl GetTRS<Translation, Rotation, Scale> for Transform {
    fn get_translation(&self) -> &Translation {
        &self.translation
    }

    fn get_rotation(&self) -> &Rotation {
        &self.rotation
    }

    fn get_scale(&self) -> &Scale {
        &self.scale
    }
}

impl GetTRSMut<Translation, Rotation, Scale> for Transform {
    fn get_translation_mut(&mut self) -> &mut Translation {
        &mut self.translation
    }

    fn get_rotation_mut(&mut self) -> &mut Rotation {
        &mut self.rotation
    }

    fn get_scale_mut(&mut self) -> &mut Scale {
        &mut self.scale
    }
}
