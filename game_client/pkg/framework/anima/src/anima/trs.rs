use super::Anima;
use crate::{Duration, Easing, GetTRS, RotationTransform, ScaleTransform, SetTRS, TranslationTransform};

impl<'trs, 's: 'trs>
    GetTRS<
        's,
        (&'trs TranslationTransform, &'trs Easing, &'trs Duration),
        (&'trs RotationTransform, &'trs Easing, &'trs Duration),
        (&'trs ScaleTransform, &'trs Easing, &'trs Duration),
    > for Anima
{
    fn get_translation(&self) -> (&TranslationTransform, &Easing, &Duration) {
        (self.transforms.get_translation(), self.easings.get_translation(), self.durations.get_translation())
    }

    fn get_rotation(&self) -> (&RotationTransform, &Easing, &Duration) {
        (self.transforms.get_rotation(), self.easings.get_rotation(), self.durations.get_rotation())
    }

    fn get_scale(&self) -> (&ScaleTransform, &Easing, &Duration) {
        (self.transforms.get_scale(), self.easings.get_scale(), self.durations.get_scale())
    }
}

pub struct Partial<TRS>(Option<TRS>, Option<Easing>, Option<Duration>);

impl SetTRS<Partial<TranslationTransform>, Partial<RotationTransform>, Partial<ScaleTransform>> for Anima {
    fn set_translation(mut self, translation: impl Into<Partial<TranslationTransform>>) -> Self {
        let Partial(transform, easing, duration) = translation.into();

        if let Some(t) = transform {
            self.transforms = self.transforms.set_translation(t);
        }

        if let Some(e) = easing {
            self.easings = self.easings.set_translation(e);
        }

        if let Some(d) = duration {
            self.durations = self.durations.set_translation(d);
        }

        self
    }

    fn set_rotation(mut self, rotation: impl Into<Partial<RotationTransform>>) -> Self {
        let Partial(transform, easing, duration) = rotation.into();

        if let Some(r) = transform {
            self.transforms = self.transforms.set_rotation(r);
        }

        if let Some(e) = easing {
            self.easings = self.easings.set_rotation(e);
        }

        if let Some(d) = duration {
            self.durations = self.durations.set_rotation(d);
        }

        self
    }

    fn set_scale(mut self, scale: impl Into<Partial<ScaleTransform>>) -> Self {
        let Partial(transform, easing, duration) = scale.into();

        if let Some(s) = transform {
            self.transforms = self.transforms.set_scale(s);
        }

        if let Some(e) = easing {
            self.easings = self.easings.set_scale(e);
        }

        if let Some(d) = duration {
            self.durations = self.durations.set_scale(d);
        }

        self
    }
}

impl<T: Into<Option<TranslationTransform>>, E: Into<Option<Easing>>, D: Into<Option<Duration>>> From<(T, E, D)>
    for Partial<TranslationTransform>
{
    fn from((translation, easing, duration): (T, E, D)) -> Self {
        Self(translation.into(), easing.into(), duration.into())
    }
}

impl<R: Into<Option<RotationTransform>>, E: Into<Option<Easing>>, D: Into<Option<Duration>>> From<(R, E, D)>
    for Partial<RotationTransform>
{
    fn from((rotation, easing, duration): (R, E, D)) -> Self {
        Self(rotation.into(), easing.into(), duration.into())
    }
}

// impl<S: Into<Option<ScaleTransform>>, E: Into<Option<Easing>>, D: Into<Option<Duration>>> From<(S, E, D)>
//     for Partial<ScaleTransform>
// {
//     fn from((scale, easing, duration): (S, E, D)) -> Self {
//         Self(scale.into(), easing.into(), duration.into())
//     }
// }
