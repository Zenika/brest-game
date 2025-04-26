use bevy::prelude::*;

use crate::card_location::{Deck, Graveyard, Hand, Played};

use super::{
    DeckSequence, DeckSequenceStamp, GraveyardSequence, GraveyardSequenceStamp, HandSequence,
    HandSequenceStamp, PlayedSequence, PlayedSequenceStamp, Sequence,
};

pub fn increment_deck_sequence(
    mut commands: Commands,
    mut sequence: ResMut<DeckSequence>,
    query: Query<Entity, Added<Deck>>,
) {
    for entity in &query {
        commands
            .entity(entity)
            .insert(DeckSequenceStamp(sequence.next()));
    }
}

pub fn increment_hand_sequence(
    mut commands: Commands,
    mut sequence: ResMut<HandSequence>,
    query: Query<Entity, Added<Hand>>,
) {
    for entity in &query {
        commands
            .entity(entity)
            .insert(HandSequenceStamp(sequence.next()));
    }
}

pub fn increment_played_sequence(
    mut commands: Commands,
    mut sequence: ResMut<PlayedSequence>,
    query: Query<Entity, Added<Played>>,
) {
    for entity in &query {
        commands
            .entity(entity)
            .insert(PlayedSequenceStamp(sequence.next()));
    }
}

pub fn increment_graveyard_sequence(
    mut commands: Commands,
    mut sequence: ResMut<GraveyardSequence>,
    query: Query<Entity, Added<Graveyard>>,
) {
    for entity in &query {
        commands
            .entity(entity)
            .insert(GraveyardSequenceStamp(sequence.next()));
    }
}
