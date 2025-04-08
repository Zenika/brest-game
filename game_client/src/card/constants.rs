use std::{
    f32::consts::{FRAC_PI_4, PI},
    sync::LazyLock,
};

use bevy::prelude::*;

pub const CARD_WIDTH: f32 = 6.3;
pub const CARD_HEIGHT: f32 = 8.8;
pub const CARD_THICKNESS: f32 = 0.12;
pub const CARD_SIZE: Vec3 = Vec3::new(CARD_WIDTH, CARD_HEIGHT, CARD_THICKNESS);

pub const GAP: f32 = 0.1 * CARD_WIDTH;

pub const HAND_CARD_Y: f32 = -30.;
pub const HAND_CARD_Z: f32 = 12.;
pub static HAND_CARD_ROTATION: LazyLock<Quat> = LazyLock::new(|| Quat::from_rotation_x(FRAC_PI_4));

pub const GRAVEYARD_CARD_X: f32 = 24.;
pub const GRAVEYARD_CARD_Y: f32 = -12.;
pub static GRAVEYARD_CARD_ROTATION: LazyLock<Quat> = LazyLock::new(|| Quat::from_rotation_y(-PI));

pub const DECK_CARD_X: f32 = 24.;
pub const DECK_CARD_Y: f32 = -24.;
pub static DECK_CARD_ROTATION: LazyLock<Quat> = LazyLock::new(|| Quat::from_rotation_y(-PI));

pub const PLAYED_CARD_X: f32 = 0.;
pub const PLAYED_CARD_Y: f32 = -12.;
pub static PLAYED_CARD_ROTATION: LazyLock<Quat> = LazyLock::new(|| Quat::from_rotation_x(0.));
