use bevy::prelude::*;

use super::*;
use crate::playground::{Deck, Field, Graveyard, Hand};

pub struct PlaygroundSequencesPlugin;

impl Plugin for PlaygroundSequencesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Sequence<Deck>>()
            .init_resource::<Sequence<Hand>>()
            .init_resource::<Sequence<Field>>()
            .init_resource::<Sequence<Graveyard>>()
            .add_systems(
                Update,
                (
                    increment_sequence::<Deck>,
                    increment_sequence::<Hand>,
                    increment_sequence::<Field>,
                    increment_sequence::<Graveyard>,
                ),
            );
    }
}
