use bevy::prelude::*;

use super::{
    components::{CardLocation, CardType},
    resources::{CardMaterials, CardMesh},
    systems::{
        cycle_location_on_click, update_material_on_pointer_out, update_material_on_pointer_over,
    },
};

pub struct SpawnCard {
    pub index: usize,
    pub card_type: CardType,
}

impl Command for SpawnCard {
    fn apply(self, world: &mut World) {
        let card_mesh = world.resource::<CardMesh>();
        let card_materials = world.resource::<CardMaterials>();

        let base_bundle = (
            Name::new(format!("Card {}", self.index)),
            self.card_type,
            CardLocation::Deck,
        );

        let rendering_bundle = (
            Transform::from_xyz(0., 0., 1.),
            Mesh3d((*card_mesh).clone()),
            MeshMaterial3d(card_materials.base.clone()),
        );

        let mut commands = world.commands();

        commands
            .spawn(base_bundle)
            .insert(rendering_bundle)
            .observe(update_material_on_pointer_over)
            .observe(update_material_on_pointer_out)
            .observe(cycle_location_on_click);
    }
}

pub trait SpawnCardExt {
    fn spawn_card(&mut self, index: usize, card_type: CardType);
}

impl SpawnCardExt for Commands<'_, '_> {
    fn spawn_card(&mut self, index: usize, card_type: CardType) {
        self.queue(SpawnCard { index, card_type });
    }
}
