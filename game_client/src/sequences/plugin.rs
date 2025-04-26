use bevy::prelude::*;

use super::*;

pub struct SequencesPlugin;

impl Plugin for SequencesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DeckSequence>()
            .init_resource::<HandSequence>()
            .init_resource::<PlayedSequence>()
            .init_resource::<GraveyardSequence>()
            .register_type::<DeckSequenceStamp>()
            .register_type::<HandSequenceStamp>()
            .register_type::<PlayedSequenceStamp>()
            .register_type::<GraveyardSequenceStamp>()
            .register_type::<DeckSequence>()
            .register_type::<HandSequence>()
            .register_type::<PlayedSequence>()
            .register_type::<GraveyardSequence>()
            .add_systems(
                Update,
                (
                    increment_deck_sequence,
                    increment_hand_sequence,
                    increment_played_sequence,
                    increment_graveyard_sequence,
                ),
            );
    }
}
