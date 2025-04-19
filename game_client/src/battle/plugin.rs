use bevy::prelude::*;

#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::StateInspectorPlugin;
use states_timer::{StatesTimer, reset_timer_on_transition};

use super::BattlePhase;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_state::<BattlePhase>()
            .register_type::<BattlePhase>()
            .register_type::<State<BattlePhase>>()
            .register_type::<NextState<BattlePhase>>();

        #[cfg(debug_assertions)]
        app.add_plugins(StateInspectorPlugin::<BattlePhase>::default());

        app.insert_resource(StatesTimer::<BattlePhase>::new(1.0))
            .register_type::<StatesTimer<BattlePhase>>();

        app.add_systems(Update, reset_timer_on_transition::<BattlePhase>());
    }
}
