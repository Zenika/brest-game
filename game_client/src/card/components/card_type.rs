use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub enum CardType {
    Attack,
    Defense,
    Spell,
}
