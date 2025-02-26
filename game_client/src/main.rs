mod card;
mod scene;

use bevy::{
    DefaultPlugins,
    app::App,
    color::palettes::{
        css::{TEAL, WHITE},
        tailwind::{ROSE_300, ROSE_600},
    },
    pbr::light_consts,
    picking::mesh_picking::MeshPickingPlugin,
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
        ))
        .add_plugins(ScenePlugin {
            key_light_illuminance: light_consts::lux::OVERCAST_DAY,
            key_light_shadows_enabled: true,
            fill_light_intensity: 200.,
            fill_light_color: TEAL.into(),
            board_material_color: WHITE.into(),
        })
        .add_plugins(CardPlugin {
            base_card_color: ROSE_300.into(),
            hover_card_color: ROSE_600.into(),
        })
        .run();
}
