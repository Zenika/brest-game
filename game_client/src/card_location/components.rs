use bevy::prelude::*;

// #[derive(Component, Debug, Reflect, PartialEq, Clone)]
// #[reflect(Component)]

// pub enum CardLocation {
//     Hand,
//     Board,
//     Graveyard,
//     Deck,
// }

pub trait CardLocation: Component + Reflect + PartialEq + Clone + Copy {}

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
pub struct Deck;
impl CardLocation for Deck {}

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
pub struct Hand;
impl CardLocation for Hand {}

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
pub struct Played;
impl CardLocation for Played {}

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
pub struct Graveyard;
impl CardLocation for Graveyard {}
