use bevy::math::Vec3;

use super::card::CardType;

pub const CARDS: [(CardType, f32, f32, f32); 3] = [
    (CardType::ATTACK, -2., -4., 1.),
    (CardType::DEFENSE, 0., -4., 1.),
    (CardType::SPELL, 2., -4., 1.),
];

pub const CARD_SIZE: Vec3 = Vec3::new(1.0, 2.0, 0.01);
