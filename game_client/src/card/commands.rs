use bevy::prelude::*;

use super::{
    bundles::CardBundle,
    components::{CardLocation, CardType},
    resources::{CardMaterials, CardMesh},
    systems::{play_on_click, update_material_on_pointer_out, update_material_on_pointer_over},
};

pub struct SpawnCard {
    pub index: usize,
    pub card_type: CardType,
    pub x: f32,
}

impl Command for SpawnCard {
    fn apply(self, world: &mut World) {
        let card_mesh = world.resource::<CardMesh>();
        let card_materials = world.resource::<CardMaterials>();

        let bundle = CardBundle {
            name: Name::new(format!("Card {}", self.index)),
            card_type: self.card_type,
            card_location: CardLocation::HAND,
            mesh: Mesh3d((*card_mesh).clone()),
            mesh_material: MeshMaterial3d(card_materials.base.clone()),
            transform: Transform::from_xyz(self.x, 0., 0.),
        };

        world
            .commands()
            .spawn(bundle)
            .observe(update_material_on_pointer_over)
            .observe(update_material_on_pointer_out)
            .observe(play_on_click);
    }
}

pub trait SpawnCardExt {
    fn spawn_card(&mut self, index: usize, card_type: CardType, x: f32);
}

impl<'w, 's> SpawnCardExt for Commands<'w, 's> {
    fn spawn_card(&mut self, index: usize, card_type: CardType, x: f32) {
        self.queue(SpawnCard {
            index,
            card_type,
            x,
        });
    }
}
