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

#[derive(Component, Reflect, PartialEq, Eq, PartialOrd, Ord)]
#[reflect(Component)]
pub struct DeckSeqStamp(pub u8);

#[derive(Component, Reflect, PartialEq, Eq, PartialOrd, Ord)]
#[reflect(Component)]
pub struct HandSeqStamp(pub u8);

#[derive(Component, Reflect, PartialEq, Eq, PartialOrd, Ord)]
#[reflect(Component)]
pub struct GraveyardSeqStamp(pub u8);
