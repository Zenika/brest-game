use bevy::prelude::*;

use crate::card_location::{Graveyard, Hand, Played};

pub fn on_round_ending(
    mut commands: Commands,
    played_query: Query<Entity, With<Played>>,
    hand_query: Query<Entity, With<Hand>>,
) {
    played_query.iter().for_each(|entity| {
        commands.entity(entity).remove::<Played>().insert(Graveyard);
    });

    hand_query.iter().for_each(|entity| {
        commands.entity(entity).remove::<Hand>().insert(Graveyard);
    });
}
