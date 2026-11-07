use anima::AnimaPlugin;
use bevy::color::palettes::tailwind::{GRAY_50, ORANGE_600, ORANGE_800, TEAL_50};
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
#[allow(unused_imports, clippy::single_component_path_imports)]
#[cfg(debug_assertions)]
use bevy_dylib;
use mock_server::MockServer;

#[cfg(debug_assertions)]
use super::DebugPlugin;
use super::{GluePlugin, SetupPlugin};
use crate::card::{CardColorPlugin, CardMaterialPlugin, CardMeshPlugin};
use crate::flow::{BattlePlugin, BattleRoundPlugin, RoundPlugin, TurnPlugin};
use crate::misc::constants::CARD_SIZE;
use crate::playground::{PlaygroundLocationPlugin, PlaygroundSequencesPlugin};

pub fn run() {
    let mut app = App::new();

    // Base plugins (some are required by DebugPlugin)
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(1600, 900),
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        }),
        ..default()
    }));

    #[cfg(debug_assertions)]
    app.add_plugins(DebugPlugin);

    // Support plugins
    app.add_plugins((AnimaPlugin, MeshPickingPlugin));

    // Game plugins
    app.add_plugins((
        CardColorPlugin { base: ORANGE_600.into(), hover: ORANGE_800.into() },
        CardMaterialPlugin,
        CardMeshPlugin { card_size: CARD_SIZE },
    ))
    .add_plugins((PlaygroundLocationPlugin, PlaygroundSequencesPlugin))
    .add_plugins((BattlePlugin, BattleRoundPlugin, RoundPlugin, TurnPlugin))
    .add_plugins((
        SetupPlugin {
            key_light_illuminance: light_consts::lux::OVERCAST_DAY,
            key_light_shadows_enabled: true,
            fill_light_intensity: 100.,
            fill_light_color: TEAL_50.into(),
            board_color: GRAY_50.into(),
        },
        GluePlugin,
    ))
    .add_plugins(MockServer);

    app.run();
}
