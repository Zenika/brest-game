use bevy::prelude::*;
use states_timer::StatesTimer;

use crate::flow::{BattlePhase, RoundPhase};

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct BattleRoundCount(pub u8);

impl FromWorld for BattleRoundCount {
    fn from_world(_world: &mut World) -> Self {
        BattleRoundCount(0)
    }
}

pub struct BattleRoundPlugin;

impl Plugin for BattleRoundPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BattleRoundCount>().add_systems(
            Update,
            new_round_policy.run_if(
                in_state(BattlePhase::InProgress).and_eager(in_state(RoundPhase::Waiting).or_eager(in_state(RoundPhase::Ending))),
            ),
        );
    }
}

pub fn new_round_policy(
    time: Res<Time>,
    mut battle_round_count: ResMut<BattleRoundCount>,
    mut round_phase_timer: ResMut<StatesTimer<RoundPhase>>,
    mut battle_phase_timer: ResMut<StatesTimer<BattlePhase>>,
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
