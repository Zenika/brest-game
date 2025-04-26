use bevy::prelude::*;

mod partial;
mod trs;

pub use partial::*;

pub type Easing = EaseFunction;

type TranslationEasing = Easing;
type RotationEasing = Easing;
type ScaleEasing = Easing;

#[derive(Reflect, Clone, PartialEq)]
pub struct Easings {
    translation: TranslationEasing,
    rotation: RotationEasing,
    scale: ScaleEasing,
}

impl From<(TranslationEasing, RotationEasing, ScaleEasing)> for Easings {
    fn from(
        (translation, rotation, scale): (TranslationEasing, RotationEasing, ScaleEasing),
    ) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }
}

impl Default for Easings {
    fn default() -> Self {
        Self {
            translation: TranslationEasing::Linear,
            rotation: RotationEasing::Linear,
            scale: ScaleEasing::Linear,
        }
    }
}
