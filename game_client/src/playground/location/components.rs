use bevy::prelude::*;

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
#[component(immutable)]
#[require(Transform)]
pub struct PlaygroundLocationTag;

pub trait PlaygroundLocation: Component + Reflect + PartialEq + Clone + Copy {}

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
#[component(immutable)]
pub struct Deck;
impl PlaygroundLocation for Deck {}

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
#[component(immutable)]
pub struct Hand;
impl PlaygroundLocation for Hand {}

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
#[component(immutable)]
pub struct Field;
impl PlaygroundLocation for Field {}

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
#[component(immutable)]
pub struct Graveyard;
impl PlaygroundLocation for Graveyard {}
