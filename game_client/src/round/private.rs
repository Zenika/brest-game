use bevy::{ecs::schedule::NodeConfigs, prelude::*};

use super::{RoundPhase, RoundPhaseTimer};

pub fn reset_timer_on_transition() -> NodeConfigs<Box<(dyn System<In = (), Out = ()> + 'static)>> {
    (|mut timer: ResMut<RoundPhaseTimer>| timer.reset()).run_if(on_event::<StateTransitionEvent<RoundPhase>>)
}
