use bevy::prelude::*;

#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::StateInspectorPlugin;

use super::{RoundPhase, RoundPhaseTimer, reset_timer_on_transition};

pub struct RoundPlugin;

impl Plugin for RoundPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_state::<RoundPhase>()
            .register_type::<RoundPhase>()
            .register_type::<State<RoundPhase>>()
            .register_type::<NextState<RoundPhase>>();

        #[cfg(debug_assertions)]
        app.add_plugins(StateInspectorPlugin::<RoundPhase>::default());

        app.insert_resource(RoundPhaseTimer::new(1.0))
            .register_type::<RoundPhaseTimer>();

        app.add_systems(Update, reset_timer_on_transition());
    }
}
