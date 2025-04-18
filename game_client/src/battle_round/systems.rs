use bevy::prelude::*;

use crate::{
    battle::{BattlePhase, BattlePhaseTimer},
    round::{RoundPhase, RoundPhaseTimer},
};

use super::resources::BattleRoundCount;

pub fn new_round_policy(
    time: Res<Time>,
    mut battle_round_count: ResMut<BattleRoundCount>,
    mut round_phase_timer: ResMut<RoundPhaseTimer>,
    mut battle_phase_timer: ResMut<BattlePhaseTimer>,
    mut next_round_phase: ResMut<NextState<RoundPhase>>,
    mut next_battle_phase: ResMut<NextState<BattlePhase>>,
) {
    if battle_round_count.0 < 5 {
        if round_phase_timer.tick(time.delta()).just_finished() {
            battle_round_count.0 += 1;
            next_round_phase.set(RoundPhase::Starting);
        }
    } else if battle_phase_timer.tick(time.delta()).just_finished() {
        next_battle_phase.set(BattlePhase::Ended);
    }
}
