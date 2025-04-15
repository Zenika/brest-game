use bevy::prelude::*;

#[derive(Component, Reflect, PartialEq, Clone)]
#[reflect(Component)]
pub enum CardLocation {
    Hand,
    Board,
    Graveyard,
    Deck,
}
