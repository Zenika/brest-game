use anima::AnimaPlugin;
use bevy::{
    color::palettes::tailwind::{GRAY_50, ROSE_600, ROSE_800, TEAL_50},
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    window::PresentMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{card_color::CardColorPlugin, card_material::CardMaterialPlugin};
use crate::{card_location::CardLocationPlugin, glue::GluePlugin, setup::SetupPlugin};
use crate::{card_mesh::CardMeshPlugin, scene::ScenePlugin};
use crate::{sequences::SequencesPlugin, turn::TurnsPlugin};

pub fn run() {
    App::new()
        .add_plugins((
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
            WorldInspectorPlugin::new(),
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    text_color: bevy::color::palettes::css::WHITE.into(),
                    enabled: true,
                },
            },
        ))
        .add_plugins(ScenePlugin {
            key_light_illuminance: light_consts::lux::OVERCAST_DAY,
            key_light_shadows_enabled: true,
            fill_light_intensity: 100.,
            fill_light_color: TEAL_50.into(),
            board_color: GRAY_50.into(),
        })
        .add_plugins(SetupPlugin)
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
        .add_plugins(TurnsPlugin)
        .add_plugins(GluePlugin)
        .run();
}
