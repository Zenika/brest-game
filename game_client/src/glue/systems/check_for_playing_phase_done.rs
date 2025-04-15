use bevy::prelude::*;

use crate::{
    glue::states::{OpponentPlayed, PlayerPlayed},
    turn::TurnPhase,
};

pub fn check_for_playing_phase_done(
    player_played: Res<State<PlayerPlayed>>,
    opponent_played: Res<State<OpponentPlayed>>,
    mut next_state: ResMut<NextState<TurnPhase>>,
) {
    if *player_played == PlayerPlayed::Yes && *opponent_played == OpponentPlayed::Yes {
        next_state.set(TurnPhase::Resolving);
    }
}
