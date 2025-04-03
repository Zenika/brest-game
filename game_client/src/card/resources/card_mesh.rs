use bevy::prelude::*;

use crate::card::constants::CARD_SIZE;

#[derive(Resource, Deref, Reflect)]
#[reflect(Resource)]
pub struct CardMesh(Handle<Mesh>);

impl FromWorld for CardMesh {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        CardMesh(meshes.add(Cuboid::from_size(CARD_SIZE)))
    }
}
