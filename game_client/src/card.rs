use bevy::prelude::*;

#[derive(Component, Default)]
#[require(CardType, CardLocation)]
pub struct Card;

#[derive(Component)]
pub enum CardType {
    ATTACK,
    DEFENSE,
    SPELL,
}

impl Default for CardType {
    fn default() -> Self {
        CardType::ATTACK
    }
}

#[derive(Component, Default, Reflect)]
pub struct CardLocation(pub CardLocationType);

#[derive(Reflect)]
pub enum CardLocationType {
    HAND,
    BOARD,
    GRAVEYARD,
    DECK
}

impl Default for CardLocationType {
    fn default() -> Self {
        CardLocationType::HAND
    }
}
