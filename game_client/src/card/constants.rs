use std::{
    f32::consts::{FRAC_PI_4, PI},
    sync::LazyLock,
};

use bevy::prelude::*;

pub const CARD_WIDTH: f32 = 1.;
pub const CARD_HEIGHT: f32 = 1.5;
pub const CARD_THICKNESS: f32 = 0.01;
pub const CARD_SIZE: Vec3 = Vec3::new(CARD_WIDTH, CARD_HEIGHT, CARD_THICKNESS);

pub const GAP: f32 = 0.1 * CARD_WIDTH;

pub const HAND_CARD_Y: f32 = -5.;
pub const HAND_CARD_Z: f32 = 2.;
pub static HAND_CARD_ROTATION: LazyLock<Quat> = LazyLock::new(|| Quat::from_rotation_x(FRAC_PI_4));

pub const GRAVEYARD_CARD_X: f32 = 4.;
pub const GRAVEYARD_CARD_Y: f32 = -2.;
pub static GRAVEYARD_CARD_ROTATION: LazyLock<Quat> = LazyLock::new(|| Quat::from_rotation_y(-PI));

pub const DECK_CARD_X: f32 = 4.;
pub const DECK_CARD_Y: f32 = -4.;
pub static DECK_CARD_ROTATION: LazyLock<Quat> = LazyLock::new(|| Quat::from_rotation_y(-PI));

pub const PLAYED_CARD_X: f32 = 0.;
pub const PLAYED_CARD_Y: f32 = -2.;
pub static PLAYED_CARD_ROTATION: LazyLock<Quat> = LazyLock::new(|| Quat::from_rotation_x(0.));
