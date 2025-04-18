use bevy::{ecs::schedule::NodeConfigs, prelude::*};

use super::{BattlePhase, BattlePhaseTimer};

pub fn reset_timer_on_transition() -> NodeConfigs<Box<(dyn System<In = (), Out = ()> + 'static)>> {
    (|mut timer: ResMut<BattlePhaseTimer>| timer.reset())
        .run_if(on_event::<StateTransitionEvent<BattlePhase>>)
}
