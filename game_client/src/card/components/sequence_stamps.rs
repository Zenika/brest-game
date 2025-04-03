use bevy::prelude::*;

#[derive(Component, Reflect, PartialEq, Eq, PartialOrd, Ord)]
#[reflect(Component)]
pub struct DeckSequenceStamp(pub u8);

#[derive(Component, Reflect, PartialEq, Eq, PartialOrd, Ord)]
#[reflect(Component)]
pub struct HandSequenceStamp(pub u8);

#[derive(Component, Reflect, PartialEq, Eq, PartialOrd, Ord)]
#[reflect(Component)]
pub struct GraveyardSequenceStamp(pub u8);
