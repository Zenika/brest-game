use bevy::prelude::*;

use crate::{battle::BattlePhase, round::RoundPhase};

use super::{BattleRoundCount, new_round_policy};

pub struct BattleRoundPlugin;

impl Plugin for BattleRoundPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<BattleRoundCount>()
            .register_type::<BattleRoundCount>()
            .add_systems(
                Update,
                new_round_policy.run_if(
                    in_state(BattlePhase::InProgress)
                        .and(in_state(RoundPhase::Waiting).or(in_state(RoundPhase::Ending))),
                ),
            );
    }
}
