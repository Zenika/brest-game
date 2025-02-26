use bevy::{
    asset::Assets,
    color::Color,
    core::Name,
    core_pipeline::core_3d::Camera3d,
    ecs::system::{Commands, ResMut},
    math::primitives::Rectangle,
    pbr::{DirectionalLight, MeshMaterial3d, StandardMaterial},
    render::mesh::{Mesh, Mesh3d},
    transform::components::Transform,
    utils::default,
};

use super::constants::{BOARD_SIDE_LENGTH, CAMERA_POSITION, CAMERA_TARGET, CAMERA_UP};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Transform::from_translation(CAMERA_POSITION).looking_at(CAMERA_TARGET, CAMERA_UP),
    ));
}

pub fn setup_key_light(illuminance: f32, shadows_enabled: bool) -> impl Fn(Commands<'_, '_>) {
    move |mut commands: Commands| {
        commands.spawn((
            Name::new("Key light"),
            DirectionalLight {
                illuminance,
                shadows_enabled,
                ..default()
            },
        ));
    }
}

pub fn setup_board(
    material_color: Color,
) -> impl Fn(Commands<'_, '_>, ResMut<'_, Assets<Mesh>>, ResMut<'_, Assets<StandardMaterial>>) {
    move |mut commands: Commands,
          mut meshes: ResMut<Assets<Mesh>>,
          mut materials: ResMut<Assets<StandardMaterial>>| {
        commands.spawn((
            Name::new("Board"),
            Mesh3d(meshes.add(Rectangle::from_length(BOARD_SIDE_LENGTH))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: material_color,
                ..default()
            })),
        ));
    }
}
