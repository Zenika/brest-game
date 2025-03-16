use super::components::CardType;

pub const CARDS: [(CardType, f32); 3] = [
    (CardType::Attack, -2.),
    (CardType::Defense, 0.),
    (CardType::Spell, 2.),
];
