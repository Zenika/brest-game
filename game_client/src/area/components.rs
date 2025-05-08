use bevy::prelude::*;

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
#[component(immutable)]
pub struct Player;

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
#[component(immutable)]
pub struct Opponent;

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
#[component(immutable)]
#[require(Transform)]
pub struct PlaygroundArea;

pub trait Area: Component + Reflect + PartialEq + Clone + Copy {}

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
#[component(immutable)]
pub struct Deck;
impl Area for Deck {}

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
#[component(immutable)]
pub struct Hand;
impl Area for Hand {}

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
#[component(immutable)]
pub struct Played;
impl Area for Played {}

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
#[component(immutable)]
pub struct Graveyard;
impl Area for Graveyard {}
