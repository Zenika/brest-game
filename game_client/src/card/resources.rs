use bevy::prelude::*;

use super::constants::CARD_SIZE;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct CardColors {
    pub base: Color,
    pub over: Color,
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct CardMaterials {
    pub base: Handle<StandardMaterial>,
    pub over: Handle<StandardMaterial>,
}

impl FromWorld for CardMaterials {
    fn from_world(world: &mut World) -> Self {
        let &CardColors { base, over } = world.resource::<CardColors>();
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

        CardMaterials {
            base: materials.add(StandardMaterial {
                base_color: base,
                ..default()
            }),
            over: materials.add(StandardMaterial {
                base_color: over,
                ..default()
            }),
        }
    }
}

#[derive(Resource, Deref, Reflect)]
#[reflect(Resource)]
pub struct CardMesh(Handle<Mesh>);

impl FromWorld for CardMesh {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        CardMesh(meshes.add(Cuboid::from_size(CARD_SIZE)))
    }
}

#[derive(Resource, Deref, Reflect)]
#[reflect(Resource)]
pub struct DeckSeq(u8);

impl DeckSeq {
    pub fn next(&mut self) -> u8 {
        self.0 += 1;

        self.0
    }
}

impl FromWorld for DeckSeq {
    fn from_world(_: &mut World) -> Self {
        Self(0)
    }
}

#[derive(Resource, Deref, Reflect)]
#[reflect(Resource)]
pub struct HandSeq(u8);

impl HandSeq {
    pub fn next(&mut self) -> u8 {
        self.0 += 1;

        self.0
    }
}

impl FromWorld for HandSeq {
    fn from_world(_: &mut World) -> Self {
        Self(0)
    }
}

#[derive(Resource, Deref, Reflect)]
#[reflect(Resource)]
pub struct GraveyardSeq(u8);

impl GraveyardSeq {
    pub fn next(&mut self) -> u8 {
        self.0 += 1;

        self.0
    }
}

impl FromWorld for GraveyardSeq {
    fn from_world(_: &mut World) -> Self {
        Self(0)
    }
}
