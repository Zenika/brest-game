use bevy::prelude::*;

use crate::card_location::CardLocation;

use super::{
    DeckSequence, DeckSequenceStamp, GraveyardSequence, GraveyardSequenceStamp, HandSequence,
    HandSequenceStamp,
};

pub fn increment_sequences(
    mut commands: Commands,
    mut deck_seq: ResMut<DeckSequence>,
    mut hand_seq: ResMut<HandSequence>,
    mut graveyard_seq: ResMut<GraveyardSequence>,
    query: Query<(Entity, &CardLocation), Changed<CardLocation>>,
) {
    for (entity, location) in &query {
        match *location {
            CardLocation::Hand => {
                commands
                    .entity(entity)
                    .insert(HandSequenceStamp(hand_seq.next()));
            }
            CardLocation::Board => {}
            CardLocation::Graveyard => {
                commands
                    .entity(entity)
                    .insert(GraveyardSequenceStamp(graveyard_seq.next()));
            }
            CardLocation::Deck => {
                commands
                    .entity(entity)
                    .insert(DeckSequenceStamp(deck_seq.next()));
            }
        }
    }
}
