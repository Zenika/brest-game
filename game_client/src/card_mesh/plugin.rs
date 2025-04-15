use bevy::prelude::*;

use super::CardMesh;

pub struct CardMeshPlugin;

impl Plugin for CardMeshPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CardMesh>().register_type::<CardMesh>();
    }
}
