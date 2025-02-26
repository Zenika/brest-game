use bevy::{
    asset::{Assets, Handle},
    color::Color,
    core::Name,
    ecs::{
        observer::Trigger,
        query::Changed,
        system::{Commands, Query, ResMut},
    },
    math::primitives::Cuboid,
    pbr::{MeshMaterial3d, StandardMaterial},
    picking::events::{Out, Over, Pointer, Up},
    render::mesh::{Mesh, Mesh3d},
    transform::components::Transform,
};

use super::{
    card::{Card, CardLocation},
    constants::{CARD_SIZE, CARDS},
};

pub fn setup_cards(
    base_color: Color,
    hover_color: Color,
) -> impl Fn(Commands<'_, '_>, ResMut<'_, Assets<Mesh>>, ResMut<'_, Assets<StandardMaterial>>) {
    move |mut commands: Commands,
          mut meshes: ResMut<Assets<Mesh>>,
          mut materials: ResMut<Assets<StandardMaterial>>| {
        let card_base_material = materials.add(base_color);
        let card_hover_material = materials.add(hover_color);
        let card_mesh = meshes.add(Cuboid::from_size(CARD_SIZE));

        for (index, (card_type, x, y, z)) in CARDS.into_iter().enumerate() {
            commands
                .spawn((
                    Name::new(format!("Card {}", index)),
                    Card,
                    card_type,
                    Mesh3d(card_mesh.clone()),
                    MeshMaterial3d(card_base_material.clone()),
                    Transform::from_xyz(x, y, z),
                ))
                .observe(update_material_on::<Pointer<Over>>(
                    card_hover_material.clone(),
                ))
                .observe(update_material_on::<Pointer<Out>>(
                    card_base_material.clone(),
                ))
                .observe(play_on_click);
        }
    }
}

pub fn update_material_on<E>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(Trigger<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    move |trigger, mut query| {
        if let Ok(mut material) = query.get_mut(trigger.entity()) {
            material.0 = new_material.clone();
        }
    }
}

pub fn update_card_position(
    mut query: Query<(&CardLocation, &mut Transform), Changed<CardLocation>>,
) {
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

fn play_on_click(trigger: Trigger<Pointer<Up>>, mut query: Query<&mut CardLocation>) {
    if let Ok(mut location) = query.get_mut(trigger.entity()) {
        *location = CardLocation::BOARD;
    }
}
