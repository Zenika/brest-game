use bevy::reflect::Reflect;

mod partial;
mod trs;

pub use partial::*;

pub type Duration = f32;

type TranslationDuration = Duration;
type RotationDuration = Duration;
type ScaleDuration = Duration;

#[derive(Reflect, Clone, PartialEq)]
pub struct Durations {
    translation: TranslationDuration,
    rotation: RotationDuration,
    scale: ScaleDuration,
}

impl From<(TranslationDuration, RotationDuration, ScaleDuration)> for Durations {
    fn from(
        (transform, rotation, scale): (TranslationDuration, RotationDuration, ScaleDuration),
    ) -> Self {
        Self {
            translation: transform,
            rotation,
            scale,
        }
    }
}

impl Default for Durations {
    fn default() -> Self {
        Self {
            translation: 1.,
            rotation: 1.,
            scale: 1.,
        }
    }
}
