use bevy::prelude::*;

#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct CardID(pub u8);

#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct ContestantID(pub u8);
