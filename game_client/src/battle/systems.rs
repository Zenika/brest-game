use bevy::{ecs::schedule::NodeConfigs, prelude::*};

use super::{BattlePhase, BattlePhaseTimer};

pub fn set_phase(phase: &BattlePhase) -> impl Fn(ResMut<NextState<BattlePhase>>) {
    |mut next_state: ResMut<NextState<BattlePhase>>| next_state.set(phase.clone())
}

// Aliases to shorten the closure header (hard to read when 3 lines long)
type RTime<'a> = Res<'a, Time>;
type RMTimer<'a> = ResMut<'a, BattlePhaseTimer>;
type RMNextState<'a> = ResMut<'a, NextState<BattlePhase>>;

pub fn transition_on_timer_policy(
    from: &'static BattlePhase,
    to: &'static BattlePhase,
) -> NodeConfigs<Box<(dyn System<In = (), Out = ()> + 'static)>> {
    (move |time: RTime, mut timer: RMTimer, mut next_state: RMNextState| {
        if timer.tick(time.delta()).just_finished() {
            next_state.set(to.clone());
        }
    })
    .run_if(in_state(from.clone()))
}
