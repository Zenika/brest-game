use bevy::prelude::*;

use super::{
    bundles::CardBundle,
    components::CardLocation,
    fixtures::CARDS,
    resources::{CardMaterials, CardMesh},
};

type Click = Pointer<Up>;

pub fn setup_cards(
    mut commands: Commands,
    card_materials: Res<CardMaterials>,
    card_mesh: Res<CardMesh>,
) {
    for (index, (card_type, x)) in CARDS.into_iter().enumerate() {
        spawn_card_bundle(
            &mut commands,
            CardBundle {
                name: Name::new(format!("Card {}", index)),
                card_type,
                card_location: CardLocation::HAND,
                mesh: Mesh3d(card_mesh.clone()),
                mesh_material: MeshMaterial3d(card_materials.base.clone()),
                transform: Transform::from_xyz(x, 0., 0.),
            },
        );
    }
}

fn spawn_card_bundle(commands: &mut Commands, bundle: CardBundle) {
    commands
        .spawn(bundle)
        .observe(update_material_on_pointer_over)
        .observe(update_material_on_pointer_out)
        .observe(play_on_click);
}

fn update_material_on_pointer_over(
    trigger: Trigger<Pointer<Over>>,
    card_materials: Res<CardMaterials>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    if let Ok(mut material) = query.get_mut(trigger.entity()) {
        material.0 = card_materials.over.clone();
    }
}

fn update_material_on_pointer_out(
    trigger: Trigger<Pointer<Out>>,
    card_materials: Res<CardMaterials>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    if let Ok(mut material) = query.get_mut(trigger.entity()) {
        material.0 = card_materials.base.clone();
    }
}

fn play_on_click(trigger: Trigger<Click>, mut query: Query<&mut CardLocation>) {
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
