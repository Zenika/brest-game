use crate::card::{
    components::{CardLocation, DeckSequenceStamp},
    events::DrawEvent,
    states::{OpponentPlayed, PlayerPlayed, TurnState},
};

use bevy::prelude::*;

pub fn on_turn_starting(
    mut query: Query<(&CardLocation, &DeckSequenceStamp, Entity)>,
    mut draw_events: EventWriter<DrawEvent>,
    mut next_state: ResMut<NextState<TurnState>>,
    mut next_player_played: ResMut<NextState<PlayerPlayed>>,
    mut next_opponent_played: ResMut<NextState<OpponentPlayed>>,
) {
    let target_location = CardLocation::Deck;

    let cards = query.iter_mut();

    let mut card_vec: Vec<_> = cards
        .filter(|&(location, _, _)| *location == target_location)
        .collect();

    card_vec.sort_by(|(_, seq_stamp_a, _), (_, seq_stamp_b, _)| seq_stamp_b.cmp(seq_stamp_a));

    card_vec.into_iter().take(3).for_each(|(_, _, entity)| {
        draw_events.send(DrawEvent { entity });
    });

    next_state.set(TurnState::Playing);
    next_player_played.set(PlayerPlayed::No);
    next_opponent_played.set(OpponentPlayed::No);
}

pub fn on_ending(mut query: Query<&mut CardLocation>) {
    query.iter_mut().for_each(|mut location| {
        if *location == CardLocation::Board || *location == CardLocation::Hand {
            *location = CardLocation::Graveyard;
        }
    });
}
