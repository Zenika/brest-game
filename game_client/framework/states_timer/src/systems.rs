use bevy::{ecs::schedule::NodeConfigs, prelude::*, state::state::FreelyMutableState};

use super::StatesTimer;

pub fn reset_timer_on_transition<S: States>()
-> NodeConfigs<Box<(dyn System<In = (), Out = ()> + 'static)>> {
    (|mut timer: ResMut<StatesTimer<S>>| timer.reset()).run_if(on_event::<StateTransitionEvent<S>>)
}

// Alias to shorten the closure header (hard to read when 3 lines long)
type RTime<'a> = Res<'a, Time>;

pub fn set_on_timer<S: FreelyMutableState + Copy>(
    from: S,
    to: S,
) -> NodeConfigs<Box<(dyn System<In = (), Out = ()>)>> {
    (move |time: RTime, mut timer: ResMut<StatesTimer<S>>, mut next_state: ResMut<NextState<S>>| {
        if timer.tick(time.delta()).just_finished() {
            next_state.set(to);
        }
    })
    .run_if(in_state(from))
}
