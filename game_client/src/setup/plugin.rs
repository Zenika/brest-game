use bevy::prelude::*;

use super::{BoardColor, setup_board, setup_camera, setup_cards, setup_key_light};

pub struct SetupPlugin {
    pub key_light_illuminance: f32,
    pub key_light_shadows_enabled: bool,
    pub fill_light_color: Color,
    pub fill_light_intensity: f32,
    pub board_color: Color,
}

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            color: self.fill_light_color,
            brightness: self.fill_light_intensity,
        })
        .insert_resource(BoardColor(self.board_color))
        .add_systems(
            Startup,
            (
                setup_camera,
                // Parameterized system without SystemParam, cf `systems.rs`
                setup_key_light(self.key_light_illuminance, self.key_light_shadows_enabled),
                // Parameterized system with SystemParam, cf `systems.rs`
                setup_board,
                setup_cards,
            ),
        );
    }
}
