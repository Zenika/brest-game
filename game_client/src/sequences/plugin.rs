use bevy::prelude::*;

use super::*;

pub struct SequencesPlugin;

impl Plugin for SequencesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DeckSequence>()
            .init_resource::<HandSequence>()
            .init_resource::<GraveyardSequence>()
            .register_type::<DeckSequenceStamp>()
            .register_type::<HandSequenceStamp>()
            .register_type::<GraveyardSequenceStamp>()
            .register_type::<DeckSequence>()
            .register_type::<HandSequence>()
            .register_type::<GraveyardSequence>()
            .add_systems(Update, increment_sequences);
    }
}
