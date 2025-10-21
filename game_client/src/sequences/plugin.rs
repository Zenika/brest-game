use bevy::prelude::*;

use crate::area::{Deck, Graveyard, Hand, Played};

use super::*;

pub struct SequencesPlugin;

impl Plugin for SequencesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Sequence<Deck>>()
            .init_resource::<Sequence<Hand>>()
            .init_resource::<Sequence<Played>>()
            .init_resource::<Sequence<Graveyard>>()
            // .register_type::<SequenceStamp<Deck>>()
            // .register_type::<SequenceStamp<Hand>>()
            // .register_type::<SequenceStamp<Played>>()
            // .register_type::<SequenceStamp<Graveyard>>()
            // .register_type::<Sequence<Deck>>()
            // .register_type::<Sequence<Hand>>()
            // .register_type::<Sequence<Played>>()
            // .register_type::<Sequence<Graveyard>>()
            .add_systems(
                Update,
                (
                    increment_sequence::<Deck>,
                    increment_sequence::<Hand>,
                    increment_sequence::<Played>,
                    increment_sequence::<Graveyard>,
                ),
            );
    }
}
