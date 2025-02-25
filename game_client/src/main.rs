//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plateau
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(5.0, 10.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // card
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 0.01))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255,165,0))),
        Transform::from_xyz(-2., -4., 0.5),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 0.01))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255,165,0))),
        Transform::from_xyz(-0., -4., 0.5),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 0.01))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255,165,0))),
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