use super::PartialEasings;
use crate::{Easing, GetTRS};

impl GetTRS<Option<Easing>, Option<Easing>, Option<Easing>> for PartialEasings {
    fn get_translation(&self) -> &Option<Easing> {
        &self.translation
    }

    fn get_rotation(&self) -> &Option<Easing> {
        &self.rotation
    }

    fn get_scale(&self) -> &Option<Easing> {
        &self.scale
    }
}
