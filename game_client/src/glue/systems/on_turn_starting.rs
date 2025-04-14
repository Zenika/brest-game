use crate::{
    card_location::CardLocation,
    glue::{
        events::DrawEvent,
        states::{OpponentPlayed, PlayerPlayed},
    },
    sequences::DeckSequenceStamp,
    turn::TurnPhase,
};

use bevy::prelude::*;

pub fn on_turn_starting(
    mut query: Query<(&CardLocation, &DeckSequenceStamp, Entity)>,
    mut draw_events: EventWriter<DrawEvent>,
    mut next_state: ResMut<NextState<TurnPhase>>,
    mut next_player_played: ResMut<NextState<PlayerPlayed>>,
    mut next_opponent_played: ResMut<NextState<OpponentPlayed>>,
) {
    let mut cards: Vec<_> = query
        .iter_mut()
        .filter(|&(location, _, _)| *location == CardLocation::Deck)
        .collect();

    cards.sort_by(|(_, seq_stamp_a, _), (_, seq_stamp_b, _)| seq_stamp_b.cmp(seq_stamp_a));

    cards.into_iter().take(3).for_each(|(_, _, entity)| {
        draw_events.send(DrawEvent { entity });
    });

    next_state.set(TurnPhase::Playing);
    next_player_played.set(PlayerPlayed::No);
    next_opponent_played.set(OpponentPlayed::No);
}
