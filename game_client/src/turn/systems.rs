use bevy::{ecs::schedule::NodeConfigs, prelude::*};

use super::{TurnPhase, TurnPhaseTimer};

pub fn set_turn_phase(turn_phase: &TurnPhase) -> impl Fn(ResMut<NextState<TurnPhase>>) {
    |mut next_state: ResMut<NextState<TurnPhase>>| next_state.set(turn_phase.clone())
}

// Aliases to shorten the closure header (hard to read when 3 lines long)
type RTime<'a> = Res<'a, Time>;
type RMTimer<'a> = ResMut<'a, TurnPhaseTimer>;
type RMNextState<'a> = ResMut<'a, NextState<TurnPhase>>;

pub fn transition_on_timer(
    from: &'static TurnPhase,
    to: &'static TurnPhase,
) -> NodeConfigs<Box<(dyn System<In = (), Out = ()> + 'static)>> {
    (move |time: RTime, mut timer: RMTimer, mut next_state: RMNextState| {
        if timer.tick(time.delta()).just_finished() {
            next_state.set(to.clone());
        }
    })
    .run_if(in_state(from.clone()))
}
