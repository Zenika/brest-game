use crate::{Duration, Easing, Rotation, Scale, SetTRS, Translation, TranslationOrScale};

use super::Anima;

pub struct Partial<TRS>(Option<TRS>, Option<Easing>, Option<Duration>);

impl SetTRS<Partial<Translation>, Partial<Rotation>, Partial<Scale>> for Anima {
    fn set_translation(&mut self, translation: impl Into<Partial<Translation>>) -> &mut Self {
        let Partial(translation, easing, duration) = translation.into();

        if let Some(t) = Into::<Option<Translation>>::into(translation) {
            self.transform.set_translation(t);
        }

        if let Some(e) = Into::<Option<Easing>>::into(easing) {
            self.easings.set_translation(e);
        }

        if let Some(d) = Into::<Option<Duration>>::into(duration) {
            self.durations.set_translation(d);
        }

        self
    }

    fn set_rotation(&mut self, rotation: impl Into<Partial<Rotation>>) -> &mut Self {
        let Partial(rotation, easing, duration) = rotation.into();

        if let Some(r) = Into::<Option<Rotation>>::into(rotation) {
            self.transform.set_rotation(r);
        }

        if let Some(e) = Into::<Option<Easing>>::into(easing) {
            self.easings.set_rotation(e);
        }

        if let Some(d) = Into::<Option<Duration>>::into(duration) {
            self.durations.set_rotation(d);
        }

        self
    }

    fn set_scale(&mut self, scale: impl Into<Partial<Scale>>) -> &mut Self {
        let Partial(scale, easing, duration) = scale.into();

        if let Some(s) = Into::<Option<Scale>>::into(scale) {
            self.transform.set_scale(s);
        }

        if let Some(e) = Into::<Option<Easing>>::into(easing) {
            self.easings.set_scale(e);
        }

        if let Some(d) = Into::<Option<Duration>>::into(duration) {
            self.durations.set_scale(d);
        }

        self
    }
}

impl<TS: Into<Option<TranslationOrScale>>, E: Into<Option<Easing>>, D: Into<Option<Duration>>>
    From<(TS, E, D)> for Partial<Translation>
{
    fn from((rotation, easing, duration): (TS, E, D)) -> Self {
        Self(rotation.into(), easing.into(), duration.into())
    }
}

impl<R: Into<Option<Rotation>>, E: Into<Option<Easing>>, D: Into<Option<Duration>>> From<(R, E, D)>
    for Partial<Rotation>
{
    fn from((rotation, easing, duration): (R, E, D)) -> Self {
        Self(rotation.into(), easing.into(), duration.into())
    }
}
