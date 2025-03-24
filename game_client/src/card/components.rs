use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub enum CardType {
    Attack,
    Defense,
    Spell,
}

#[derive(Component, Reflect, PartialEq)]
#[reflect(Component)]
pub enum CardLocation {
    Hand,
    Board,
    Graveyard,
    Deck,
}
