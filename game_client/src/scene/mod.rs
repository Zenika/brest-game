mod constants;
mod systems;

use bevy::{
    app::{Plugin, Startup},
    color::Color,
    pbr::AmbientLight,
};

use systems::{setup_board, setup_camera, setup_key_light};

pub struct ScenePlugin {
    pub key_light_illuminance: f32,
    pub key_light_shadows_enabled: bool,
    pub fill_light_color: Color,
    pub fill_light_intensity: f32,
    pub board_material_color: Color,
}

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(AmbientLight {
            color: self.fill_light_color,
            brightness: self.fill_light_intensity,
        })
        .add_systems(
            Startup,
            (
                setup_camera,
                // Parameterized systems without SystemParam, it could be done with Resource and SystemParam, cf. Bevy doc.
                setup_key_light(self.key_light_illuminance, self.key_light_shadows_enabled),
                setup_board(self.board_material_color),
            ),
        );
    }
}
