use bevy::prelude::*;

#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::StateInspectorPlugin;

use super::{ContestantPlayed, OpponentPlayed, PlayerPlayed};

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<ContestantPlayed>();

        app.init_state::<PlayerPlayed>()
            .register_type::<PlayerPlayed>()
            .register_type::<State<PlayerPlayed>>()
            .register_type::<NextState<PlayerPlayed>>();

        #[cfg(debug_assertions)]
        app.add_plugins(StateInspectorPlugin::<PlayerPlayed>::default());

        app.init_state::<OpponentPlayed>()
            .register_type::<OpponentPlayed>()
            .register_type::<State<OpponentPlayed>>()
            .register_type::<NextState<OpponentPlayed>>();

        #[cfg(debug_assertions)]
        app.add_plugins(StateInspectorPlugin::<OpponentPlayed>::default());
    }
}
