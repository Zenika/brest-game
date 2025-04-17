use bevy::prelude::*;

#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::StateInspectorPlugin;

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
    }
}
