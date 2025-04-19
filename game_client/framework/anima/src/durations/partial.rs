use super::{Duration, Durations};

mod trs;

pub struct PartialDurations {
    pub translation: Option<Duration>,
    pub rotation: Option<Duration>,
    pub scale: Option<Duration>,
}

impl<T: Into<Option<Duration>>, R: Into<Option<Duration>>, S: Into<Option<Duration>>>
    From<(T, R, S)> for PartialDurations
{
    fn from((translation, rotation, scale): (T, R, S)) -> Self {
        Self {
            translation: translation.into(),
            rotation: rotation.into(),
            scale: scale.into(),
        }
    }
}

impl From<Durations> for PartialDurations {
    fn from(durations: Durations) -> Self {
        Self {
            translation: durations.translation.into(),
            rotation: durations.rotation.into(),
            scale: durations.scale.into(),
        }
    }
}
