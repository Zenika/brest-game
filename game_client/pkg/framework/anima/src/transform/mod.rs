use bevy::math::{Quat, Vec3};

pub use bevy::prelude::Transform;

mod partial;
mod trs;

pub use partial::*;

pub type TranslationOrScale = Vec3;
pub type Translation = TranslationOrScale;
pub type Rotation = Quat;
pub type Scale = TranslationOrScale;
