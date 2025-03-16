use bevy::prelude::*;

use super::{
    commands::SpawnCardExt, components::CardLocation, fixtures::CARDS, resources::CardMaterials,
};

type Click = Pointer<Up>;

pub fn setup_cards(mut commands: Commands) {
    for (index, (card_type, x)) in CARDS.into_iter().enumerate() {
        commands.spawn_card(index, card_type, x);
        // commands.queue(SpawnCard {
        //     index,
        //     card_type,
        //     x,
        // });
    }
}

pub fn update_material_on_pointer_over(
    trigger: Trigger<Pointer<Over>>,
    card_materials: Res<CardMaterials>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    if let Ok(mut material) = query.get_mut(trigger.entity()) {
        material.0 = card_materials.over.clone();
    }
}

pub fn update_material_on_pointer_out(
    trigger: Trigger<Pointer<Out>>,
    card_materials: Res<CardMaterials>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    if let Ok(mut material) = query.get_mut(trigger.entity()) {
        material.0 = card_materials.base.clone();
    }
}

pub fn play_on_click(trigger: Trigger<Click>, mut query: Query<&mut CardLocation>) {
    if let Ok(mut location) = query.get_mut(trigger.entity()) {
        *location = CardLocation::BOARD;
    }
}

pub fn update_position(mut query: Query<(&CardLocation, &mut Transform), Changed<CardLocation>>) {
    for (card_location, mut transform) in &mut query {
        match card_location {
            CardLocation::HAND => {
                transform.translation.z = 0.5;
                transform.translation.y = -4.;
            }
            CardLocation::BOARD => {
                transform.translation.z = 0.01;
                transform.translation.y = 0.;
            }
            CardLocation::GRAVEYARD => {}
            CardLocation::DECK => {}
        }
    }
}
