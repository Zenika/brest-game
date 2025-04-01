use crate::{
    Durations, Easings, GetTED, GetTEDMut, GetTRS, PartialDurations, PartialEasings,
    PartialTransform, SetTED, SetTRS, Transform,
};

use super::Anima;

impl GetTED<Transform, Easings, Durations> for Anima {
    fn get_transform(&self) -> &Transform {
        &self.transform
    }

    fn get_easings(&self) -> &Easings {
        &self.easings
    }

    fn get_durations(&self) -> &Durations {
        &self.durations
    }
}

impl GetTEDMut<Transform, Easings, Durations> for Anima {
    fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn get_easings_mut(&mut self) -> &mut Easings {
        &mut self.easings
    }

    fn get_durations_mut(&mut self) -> &mut Durations {
        &mut self.durations
    }
}

impl SetTED<PartialTransform, PartialEasings, PartialDurations> for Anima {
    fn set_transform(&mut self, transform: impl Into<PartialTransform>) -> &mut Self {
        let partial: PartialTransform = transform.into();

        if let &Some(t) = partial.get_translation() {
            self.transform.set_translation(t);
        }

        if let &Some(r) = partial.get_rotation() {
            self.transform.set_rotation(r);
        }

        if let &Some(s) = partial.get_scale() {
            self.transform.set_scale(s);
        }

        self
    }

    fn set_easings(&mut self, easings: impl Into<PartialEasings>) -> &mut Self {
        let partial: PartialEasings = easings.into();

        if let &Some(t) = partial.get_translation() {
            self.easings.set_translation(t);
        }

        if let &Some(r) = partial.get_rotation() {
            self.easings.set_rotation(r);
        }

        if let &Some(s) = partial.get_scale() {
            self.easings.set_scale(s);
        }

        self
    }

    fn set_durations(&mut self, durations: impl Into<PartialDurations>) -> &mut Self {
        let partial: PartialDurations = durations.into();

        if let &Some(t) = partial.get_translation() {
            self.durations.set_translation(t);
        }

        if let &Some(r) = partial.get_rotation() {
            self.durations.set_rotation(r);
        }

        if let &Some(s) = partial.get_scale() {
            self.durations.set_scale(s);
        }

        self
    }
}
