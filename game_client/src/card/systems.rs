use anima::{Anima, SetTED, WithTED};
use bevy::prelude::*;

use super::{
    commands::SpawnCardExt, components::CardLocation, fixtures::CARDS, resources::CardMaterials,
};

type Click = Pointer<Up>;

pub fn setup_cards(mut commands: Commands) {
    for (index, (card_type, x)) in CARDS.into_iter().enumerate() {
        commands.spawn_card(index, card_type, x);
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
        *location = CardLocation::Board;
    }
}

pub fn update_position(
    mut query: Query<(&CardLocation, &Transform, &mut Anima), Changed<CardLocation>>,
) {
    for (card_location, transform, mut anima) in &mut query {
        match card_location {
            CardLocation::Hand => {
                // Anima Immunable (With) TRS API, replacing the whole Anima with a new modified one (TRS = Translation / Rotation / Scale)
                // *anima = anima
                //     .with_translation((Vec3::new(transform.translation.x, -4., 2.), None, None))
                //     .with_rotation((Quat::from_rotation_x(0.5), None, None));

                // Anima Immunable (With) TED API, replacing the whole Anima with a new modified one (TED = Transform / Easings / Durations)
                *anima = anima.with_transform((
                    Vec3::new(transform.translation.x, -4., 2.),
                    Quat::from_rotation_x(0.5),
                    None,
                ));
            }
            CardLocation::Board => {
                // Anima Mutable (Set) TRS API, modifying the anima inplace (TRS = Translation / Rotation / Scale)
                // anima
                //     .set_translation((Vec3::new(transform.translation.x, 0., 0.01), None, None))
                //     .set_rotation((Quat::from_rotation_x(0.), None, None));

                // Anima Mutable (Set) TED API, modifying the anima inplace (TED = Transform / Easings / Durations)
                anima.set_transform((
                    Vec3::new(transform.translation.x, 0., 0.01),
                    Quat::from_rotation_x(0.),
                    None,
                ));
            }
            CardLocation::Graveyard => {}
            CardLocation::Deck => {}
        }
    }
}
