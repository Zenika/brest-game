use bevy::math::{Quat, Vec3};
use bevy::reflect::Reflect;

mod partial;
mod trs;

pub use partial::*;

pub type TranslationTransform = Vec3;
pub type RotationTransform = Quat;
pub type ScaleTransform = Vec3;

#[derive(Reflect, Clone, Copy, PartialEq)]
pub struct Transforms {
    translation: TranslationTransform,
    rotation: RotationTransform,
    scale: ScaleTransform,
}

impl Default for Transforms {
    fn default() -> Self {
        Self { translation: Vec3::default(), rotation: Quat::default(), scale: Vec3::default() }
    }
}
