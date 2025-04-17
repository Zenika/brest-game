use bevy::{ecs::schedule::NodeConfigs, prelude::*};

use super::{RoundPhase, PhaseTimer};

pub fn set_phase(phase: &RoundPhase) -> impl Fn(ResMut<NextState<RoundPhase>>) {
    |mut next_state: ResMut<NextState<RoundPhase>>| next_state.set(phase.clone())
}

// Aliases to shorten the closure header (hard to read when 3 lines long)
type RTime<'a> = Res<'a, Time>;
type RMTimer<'a> = ResMut<'a, PhaseTimer>;
type RMNextState<'a> = ResMut<'a, NextState<RoundPhase>>;

pub fn transition_on_timer_policy(
    from: &'static RoundPhase,
    to: &'static RoundPhase,
) -> NodeConfigs<Box<(dyn System<In = (), Out = ()> + 'static)>> {
    (move |time: RTime, mut timer: RMTimer, mut next_state: RMNextState| {
        if timer.tick(time.delta()).just_finished() {
            next_state.set(to.clone());
        }
    })
    .run_if(in_state(from.clone()))
}
