//! A simple 3D scene with light shining over a cube sitting on a plane.

mod card;

use crate::card::{Card, CardLocation, CardLocationType, CardType};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, update_card_position)
        .register_type::<CardLocation>()
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let white_matl = materials.add(Color::WHITE);
    let orange_matl = materials.add(Color::srgb_u8(255, 165, 0));

    // plateau
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(5.0, 10.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // card
    commands.spawn((
        Card,
        CardType::ATTACK,
        Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 0.01))),
        MeshMaterial3d(orange_matl.clone()),
        Transform::from_xyz(-2., -4., 0.5),
    ));
    commands
        .spawn((
            Name::new("Card"),
            Card,
            CardType::DEFENSE,
            Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 0.01))),
            MeshMaterial3d(orange_matl.clone()),
            Transform::from_xyz(-0., -4., 0.5),
        ))
        .observe(update_material_on::<Pointer<Over>>(white_matl))
        .observe(update_material_on::<Pointer<Out>>(orange_matl.clone()))
        .observe(play_on_click());

    commands.spawn((
        Card,
        CardType::SPELL,
        Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 0.01))),
        MeshMaterial3d(orange_matl.clone()),
        Transform::from_xyz(2., -4., 0.5),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn update_material_on<E>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(Trigger<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    // An observer closure that captures `new_material`. We do this to avoid needing to write four
    // versions of this observer, each triggered by a different event and with a different hardcoded
    // material. Instead, the event type is a generic, and the material is passed in.
    move |trigger, mut query| {
        if let Ok(mut material) = query.get_mut(trigger.entity()) {
            material.0 = new_material.clone();
        }
    }
}

fn play_on_click() -> impl Fn(Trigger<Pointer<Up>>, Query<&mut CardLocation>) {
    move |trigger, mut query| {
        if let Ok(mut location) = query.get_mut(trigger.entity()) {
            location.0 = CardLocationType::BOARD;
        }
    }
}

fn update_card_position(mut query: Query<(&CardLocation, &mut Transform), Changed<CardLocation>>) {
    for (card_location, mut transform) in &mut query {
        match card_location.0 {
            CardLocationType::HAND => {
                transform.translation.z = 0.5;
                transform.translation.y = -4.;
            }
            CardLocationType::BOARD => {
                transform.translation.z = 0.01;
                transform.translation.y = 0.;
            }
            CardLocationType::GRAVEYARD => {}
            CardLocationType::DECK => {}
        }
    }
}