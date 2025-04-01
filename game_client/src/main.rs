mod card;
mod scene;

use anima::AnimaPlugin;
use bevy::{
    color::palettes::tailwind::{GRAY_50, ROSE_600, ROSE_800, TEAL_50},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use card::CardPlugin;
use scene::ScenePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            WorldInspectorPlugin::new(),
            AnimaPlugin,
        ))
        .add_plugins(ScenePlugin {
            key_light_illuminance: light_consts::lux::OVERCAST_DAY,
            key_light_shadows_enabled: true,
            fill_light_intensity: 100.,
            fill_light_color: TEAL_50.into(),
            board_color: GRAY_50.into(),
        })
        .add_plugins(CardPlugin {
            base_card_color: ROSE_800.into(),
            over_card_color: ROSE_600.into(),
        })
        .run();
}
