use bevy::{
    ecs::{
        schedule::{IntoScheduleConfigs, ScheduleConfigs, common_conditions::on_message},
        system::{Res, ResMut, System},
    },
    state::{
        condition::in_state,
        state::{
            FreelyMutableState, NextState, StateTransitionEvent as StateTransitionMessage, States,
        },
    },
    time::Time,
};

use super::StatesTimer;

pub fn reset_timer_on_transition<S: States>()
-> ScheduleConfigs<Box<dyn System<In = (), Out = ()> + 'static>> {
    (|mut timer: ResMut<StatesTimer<S>>| timer.reset())
        .run_if(on_message::<StateTransitionMessage<S>>)
}

// Alias to shorten the closure header (hard to read when 3 lines long)
type RTime<'a> = Res<'a, Time>;

pub fn set_on_timer<S: FreelyMutableState + Copy>(
    from: S,
    to: S,
) -> ScheduleConfigs<Box<dyn System<In = (), Out = ()> + 'static>> {
    (move |time: RTime, mut timer: ResMut<StatesTimer<S>>, mut next_state: ResMut<NextState<S>>| {
        if timer.tick(time.delta()).just_finished() {
            next_state.set(to);
        }
    })
    .run_if(in_state(from))
}
