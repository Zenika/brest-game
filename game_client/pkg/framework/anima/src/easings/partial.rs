use super::{Easing, Easings};

mod trs;

pub struct PartialEasings {
    pub translation: Option<Easing>,
    pub rotation: Option<Easing>,
    pub scale: Option<Easing>,
}

impl<T: Into<Option<Easing>>, R: Into<Option<Easing>>, S: Into<Option<Easing>>> From<(T, R, S)>
    for PartialEasings
{
    fn from((translation, rotation, scale): (T, R, S)) -> Self {
        Self {
            translation: translation.into(),
            rotation: rotation.into(),
            scale: scale.into(),
        }
    }
}

impl From<Easings> for PartialEasings {
    fn from(easings: Easings) -> Self {
        Self {
            translation: easings.translation.into(),
            rotation: easings.rotation.into(),
            scale: easings.scale.into(),
        }
    }
}
