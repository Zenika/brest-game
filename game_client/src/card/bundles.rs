use bevy::prelude::*;

use super::components::{CardLocation, CardType};

#[derive(Bundle)]
pub struct CardBundle {
    pub name: Name,
    pub card_type: CardType,
    pub card_location: CardLocation,
    pub mesh: Mesh3d,
    pub mesh_material: MeshMaterial3d<StandardMaterial>,
    pub transform: Transform,
}
