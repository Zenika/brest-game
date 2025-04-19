use bevy::prelude::*;

use crate::{
    round::RoundPhase,
    turn::{ContestantPlayed, OpponentPlayed, PlayerPlayed},
};

pub fn end_playing_phase_policy(
    player_played: Res<State<PlayerPlayed>>,
    opponent_played: Res<State<OpponentPlayed>>,
    mut next_state: ResMut<NextState<RoundPhase>>,
) {
    if *player_played == PlayerPlayed(ContestantPlayed::Yes)
        && *opponent_played == OpponentPlayed(ContestantPlayed::Yes)
    {
        next_state.set(RoundPhase::Resolving);
    }
}
