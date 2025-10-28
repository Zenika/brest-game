use bevy::prelude::*;

use super::CardMesh;
use crate::card::CardSize;

pub struct CardMeshPlugin {
    pub card_size: Vec3,
}

impl Plugin for CardMeshPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CardSize(self.card_size)).init_resource::<CardMesh>();
    }
}
