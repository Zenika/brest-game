use super::PartialDurations;
use crate::{Duration, GetTRS};

impl<'trs, 's: 'trs> GetTRS<'s, &'trs Option<Duration>, &'trs Option<Duration>, &'trs Option<Duration>>
    for PartialDurations
{
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
