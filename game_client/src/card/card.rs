use bevy::{ecs::component::Component, reflect::Reflect};

#[derive(Component, Default, Reflect)]
#[require(CardType, CardLocation)]
pub struct Card;

#[derive(Component, Reflect)]
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

#[derive(Component, Reflect)]
pub enum CardLocation {
    HAND,
    BOARD,
    GRAVEYARD,
    DECK,
}

impl Default for CardLocation {
    fn default() -> Self {
        CardLocation::HAND
    }
}
