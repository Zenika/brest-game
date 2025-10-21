use crate::{
    area::Deck,
    glue::messages::DrawMessage,
    round::RoundPhase,
    sequences::SequenceStamp,
    turn::{ContestantPlayed, OpponentPlayed, PlayerPlayed},
};

use bevy::prelude::*;
use shared::CardID;

pub fn on_round_starting(
    mut query: Query<(&SequenceStamp<Deck>, Entity), (With<Deck>, With<CardID>)>,
    mut draw_messages: MessageWriter<DrawMessage>,
    mut next_state: ResMut<NextState<RoundPhase>>,
    mut next_player_played: ResMut<NextState<PlayerPlayed>>,
    mut next_opponent_played: ResMut<NextState<OpponentPlayed>>,
) {
    let mut cards: Vec<_> = query.iter_mut().collect();

    cards.sort_by(|(seq_stamp_a, _), (seq_stamp_b, _)| seq_stamp_b.cmp(seq_stamp_a));

    // TODO: replace this magic number by something else
    cards.into_iter().take(3).for_each(|(_, entity)| {
        println!("{}", entity);
        draw_messages.write(DrawMessage { entity });
    });

    next_state.set(RoundPhase::Playing);
    next_player_played.set(PlayerPlayed(ContestantPlayed::No));
    next_opponent_played.set(OpponentPlayed(ContestantPlayed::No));
}
