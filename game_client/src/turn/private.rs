use bevy::{ecs::schedule::NodeConfigs, prelude::*};

use super::{TurnPhase, TurnPhaseTimer};

pub fn reset_timer_on_transition() -> NodeConfigs<Box<(dyn System<In = (), Out = ()> + 'static)>> {
    (|mut timer: ResMut<TurnPhaseTimer>| timer.reset())
        .run_if(on_event::<StateTransitionEvent<TurnPhase>>)
}
