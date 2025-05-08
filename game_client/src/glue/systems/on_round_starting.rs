use crate::{
    area::Deck,
    glue::events::DrawEvent,
    round::RoundPhase,
    sequences::DeckSequenceStamp,
    turn::{ContestantPlayed, OpponentPlayed, PlayerPlayed},
};

use bevy::prelude::*;

pub fn on_round_starting(
    mut query: Query<(&DeckSequenceStamp, Entity), With<Deck>>,
    mut draw_events: EventWriter<DrawEvent>,
    mut next_state: ResMut<NextState<RoundPhase>>,
    mut next_player_played: ResMut<NextState<PlayerPlayed>>,
    mut next_opponent_played: ResMut<NextState<OpponentPlayed>>,
) {
    let mut cards: Vec<_> = query.iter_mut().collect();

    cards.sort_by(|(seq_stamp_a, _), (seq_stamp_b, _)| seq_stamp_b.cmp(seq_stamp_a));

    // TODO: replace this magic number by something else
    cards.into_iter().take(3).for_each(|(_, entity)| {
        draw_events.write(DrawEvent { entity });
    });

    next_state.set(RoundPhase::Playing);
    next_player_played.set(PlayerPlayed(ContestantPlayed::No));
    next_opponent_played.set(OpponentPlayed(ContestantPlayed::No));
}
