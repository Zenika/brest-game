#[allow(unused_imports, clippy::single_component_path_imports)]
#[cfg(debug_assertions)]
use bevy_dylib;

use anima::AnimaPlugin;
use bevy::{
    color::palettes::tailwind::{GRAY_50, ROSE_600, ROSE_800, TEAL_50},
    prelude::*,
    window::PresentMode,
};
use mock_server::MockServer;

use crate::{
    battle::BattlePlugin, card_color::CardColorPlugin, card_location::CardLocationPlugin,
    card_material::CardMaterialPlugin, card_mesh::CardMeshPlugin, glue::GluePlugin,
    round::RoundPlugin, sequences::SequencesPlugin, setup::SetupPlugin, turn::TurnPlugin,
};

#[cfg(debug_assertions)]
use crate::debug::DebugPlugin;

pub fn run() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1600., 900.).into(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }),
        AnimaPlugin,
        MeshPickingPlugin,
    ))
    .add_plugins(SetupPlugin {
        key_light_illuminance: light_consts::lux::OVERCAST_DAY,
        key_light_shadows_enabled: true,
        fill_light_intensity: 100.,
        fill_light_color: TEAL_50.into(),
        board_color: GRAY_50.into(),
    })
    .add_plugins((
        CardColorPlugin {
            base: ROSE_800.into(),
            hover: ROSE_600.into(),
        },
        CardLocationPlugin,
        CardMaterialPlugin,
        CardMeshPlugin,
    ))
    .add_plugins(SequencesPlugin)
    .add_plugins(BattlePlugin)
    .add_plugins(RoundPlugin)
    .add_plugins(TurnPlugin)
    .add_plugins(GluePlugin)
    .add_plugins(MockServer);

    #[cfg(debug_assertions)]
    app.add_plugins(DebugPlugin);

    app.run();
}
