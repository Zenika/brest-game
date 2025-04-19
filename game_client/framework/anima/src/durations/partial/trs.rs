use crate::{Duration, GetTRS};

use super::PartialDurations;

impl GetTRS<Option<Duration>, Option<Duration>, Option<Duration>> for PartialDurations {
    fn get_translation(&self) -> &Option<Duration> {
        &self.translation
    }

    fn get_rotation(&self) -> &Option<Duration> {
        &self.rotation
    }

    fn get_scale(&self) -> &Option<Duration> {
        &self.scale
    }
}
