use std::f32::consts::PI;

use anima::{Anima, WithTED, WithTRS};
use bevy::prelude::*;

use super::{
    commands::SpawnCardExt, components::CardLocation, constants::CARD_SIZE, fixtures::CARDS,
    resources::CardMaterials,
};

type Click = Pointer<Up>;

const TRANSLATION_GAP: f32 = 0.1;

pub fn setup_cards(mut commands: Commands) {
    for (index, card_type) in CARDS.into_iter().enumerate() {
        commands.spawn_card(index, card_type);
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

pub fn cycle_location_on_click(trigger: Trigger<Click>, mut query: Query<&mut CardLocation>) {
    if let Ok(mut location) = query.get_mut(trigger.entity()) {
        match *location {
            CardLocation::Hand => *location = CardLocation::Board,
            CardLocation::Board => *location = CardLocation::Graveyard,
            CardLocation::Graveyard => *location = CardLocation::Deck,
            CardLocation::Deck => *location = CardLocation::Hand,
        }
    }
}

pub fn place_on_board(mut query: Query<(&CardLocation, &mut Anima), Changed<CardLocation>>) {
    for (card_location, mut anima) in &mut query {
        if let CardLocation::Board = card_location {
            anima.set_if_neq(anima.with_transform((
                Vec3::new(0., -2., TRANSLATION_GAP),
                Quat::from_rotation_x(0.),
                None,
            )));
        }
    }
}

pub fn arrange_deck(mut query: Query<(&CardLocation, &mut Anima)>) {
    let target_rotation = Quat::from_rotation_y(-PI);

    query
        .iter_mut()
        .filter(|&(location, _)| location == &CardLocation::Deck)
        .enumerate()
        .for_each(|(index, (_, mut anima))| {
            let target_translation = Vec3::new(4., -4., (index as f32 + 1.) * TRANSLATION_GAP);

            anima.set_if_neq(
                anima
                    .with_translation((target_translation, None, None))
                    .with_rotation((target_rotation, None, None)),
            );
        });
}

pub fn arrange_hand(mut query: Query<(&CardLocation, &mut Anima)>) {
    let target_location = CardLocation::Hand;
    let target_rotation = Quat::from_rotation_x(PI / 4.);
    let x_step = CARD_SIZE.x + TRANSLATION_GAP;

    let count = query
        .iter()
        .filter(|&(location, _)| *location == target_location)
        .count() as f32;

    query
        .iter_mut()
        .filter(|&(location, _)| *location == target_location)
        .enumerate()
        .for_each(|(index, (_, mut anima))| {
            let target_translation = Vec3::new(
                (-x_step * (count - 1.) / 2.) + (index as f32 * x_step),
                -5.,
                2.,
            );

            anima.set_if_neq(
                anima
                    .with_translation((target_translation, None, None))
                    .with_rotation((target_rotation, None, None)),
            );
        });
}

pub fn arrange_graveyard(mut query: Query<(&CardLocation, &mut Anima)>) {
    let target_rotation = Quat::from_rotation_y(-PI);

    query
        .iter_mut()
        .filter(|&(location, _)| location == &CardLocation::Graveyard)
        .enumerate()
        .for_each(|(index, (_, mut anima))| {
            let target_translation = Vec3::new(4., -2., (index as f32 + 1.) * TRANSLATION_GAP);

            anima.set_if_neq(
                anima
                    .with_translation((target_translation, None, None))
                    .with_rotation((target_rotation, None, None)),
            );
        });
}
