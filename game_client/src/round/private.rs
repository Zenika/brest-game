use bevy::{ecs::schedule::NodeConfigs, prelude::*};

use super::{RoundPhase, PhaseTimer};

pub fn reset_timer_on_transition() -> NodeConfigs<Box<(dyn System<In = (), Out = ()> + 'static)>> {
    (|mut timer: ResMut<PhaseTimer>| timer.reset()).run_if(on_event::<StateTransitionEvent<RoundPhase>>)
}
