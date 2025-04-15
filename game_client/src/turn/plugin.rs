use bevy::prelude::*;

use super::{TurnPhase, TurnPhaseTimer, reset_timer_on_transition};

pub struct TurnsPlugin;

impl Plugin for TurnsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_state::<TurnPhase>()
            .insert_resource(TurnPhaseTimer::new(1.0))
            .add_systems(Update, reset_timer_on_transition())
            .register_type::<TurnPhase>()
            .register_type::<TurnPhaseTimer>()
            .register_type::<State<TurnPhase>>()
            .register_type::<NextState<TurnPhase>>();
    }
}
