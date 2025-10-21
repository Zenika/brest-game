use bevy::prelude::*;
use shared::CardID;

use crate::area::{Graveyard, Hand, Played};

pub fn on_round_ending(
    mut commands: Commands,
    played_query: Query<Entity, (With<CardID>, With<Played>)>,
    hand_query: Query<Entity, (With<CardID>, With<Hand>)>,
) {
    played_query.iter().for_each(|entity| {
        commands.entity(entity).insert(Graveyard);
    });

    hand_query.iter().for_each(|entity| {
        commands.entity(entity).insert(Graveyard);
    });
}
