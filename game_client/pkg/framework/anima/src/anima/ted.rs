use super::Anima;
use crate::durations::PartialDurations;
use crate::easings::PartialEasings;
use crate::transforms::{PartialTransforms, Transforms};
use crate::{Durations, Easings, GetTED, SetTED, SetTRS};

impl<'ted, 's: 'ted> GetTED<'s, &'ted Transforms, &'ted Easings, &'ted Durations> for Anima {
    fn get_transforms(&self) -> &Transforms {
        &self.transforms
    }

    fn get_easings(&self) -> &Easings {
        &self.easings
    }

    fn get_durations(&self) -> &Durations {
        &self.durations
    }
}

impl SetTED<PartialTransforms, PartialEasings, PartialDurations> for Anima {
    fn set_transforms(mut self, transforms: impl Into<PartialTransforms>) -> Self {
        let partial: PartialTransforms = transforms.into();

        let new_transforms = partial.translation.map_or(self.transforms, |t| self.transforms.set_translation(t));
        let new_transforms = partial.rotation.map_or(new_transforms, |r| new_transforms.set_rotation(r));
        let new_transforms = partial.scale.map_or(new_transforms, |s| new_transforms.set_scale(s));

        self.transforms = new_transforms;
        self
    }

    fn set_easings(mut self, easings: impl Into<PartialEasings>) -> Self {
        let partial: PartialEasings = easings.into();

        let new_transforms = partial.translation.map_or(self.easings, |t| self.easings.set_translation(t));
        let new_transforms = partial.rotation.map_or(new_transforms, |r| new_transforms.set_rotation(r));
        let new_transforms = partial.scale.map_or(new_transforms, |s| new_transforms.set_scale(s));

        self.easings = new_transforms;
        self
    }

    fn set_durations(mut self, durations: impl Into<PartialDurations>) -> Self {
        let partial: PartialDurations = durations.into();

        let new_transforms = partial.translation.map_or(self.durations, |t| self.durations.set_translation(t));
        let new_transforms = partial.rotation.map_or(new_transforms, |r| new_transforms.set_rotation(r));
        let new_transforms = partial.scale.map_or(new_transforms, |s| new_transforms.set_scale(s));

        self.durations = new_transforms;
        self
    }
}
