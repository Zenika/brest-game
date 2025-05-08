use bevy::prelude::*;

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
pub struct Opponent;

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[require(Transform)]
#[reflect(Component)]
pub struct PlaygroundArea;

pub trait Area: Component + Reflect + PartialEq + Clone + Copy {}

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
pub struct Deck;
impl Area for Deck {}

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
pub struct Hand;
impl Area for Hand {}

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
pub struct Played;
impl Area for Played {}

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
pub struct Graveyard;
impl Area for Graveyard {}
