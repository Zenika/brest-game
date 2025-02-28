use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub enum CardType {
    ATTACK,
    DEFENSE,
    SPELL,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub enum CardLocation {
    HAND,
    BOARD,
    GRAVEYARD,
    DECK,
}
