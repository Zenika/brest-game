use bevy::prelude::*;

use crate::area::{Graveyard, Hand, Played, PlaygroundArea};

pub fn on_round_ending(
    mut commands: Commands,
    played_query: Query<Entity, (Without<PlaygroundArea>, With<Played>)>,
    hand_query: Query<Entity, (Without<PlaygroundArea>, With<Hand>)>,
) {
    played_query.iter().for_each(|entity| {
        commands.entity(entity).remove::<Played>().insert(Graveyard);
    });

    hand_query.iter().for_each(|entity| {
        commands.entity(entity).remove::<Hand>().insert(Graveyard);
    });
}
