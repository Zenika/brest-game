use crate::{GetTRS, Rotation, Scale, Translation};

use super::PartialTransform;

impl GetTRS<Option<Translation>, Option<Rotation>, Option<Scale>> for PartialTransform {
    fn get_translation(&self) -> &Option<Translation> {
        &self.translation
    }

    fn get_rotation(&self) -> &Option<Rotation> {
        &self.rotation
    }

    fn get_scale(&self) -> &Option<Scale> {
        &self.scale
    }
}
