use bevy::ecs::system::Commands;

use crate::{
    board_locations::{DeckPile, GraveyardPile, PlayedPile, Player},
    constants::{
        BOARD_SIDE_LENGTH, CAMERA_POSITION, CAMERA_TARGET, CAMERA_UP, CARDS_COUNT,
        PLAYER_DECK_PILE_ROTATION, PLAYER_DECK_PILE_X, PLAYER_DECK_PILE_Y,
        PLAYER_GRAVEYARD_PILE_ROTATION, PLAYER_GRAVEYARD_PILE_X, PLAYER_GRAVEYARD_PILE_Y,
        PLAYER_PLAYED_PILE_ROTATION, PLAYER_PLAYED_PILE_X, PLAYER_PLAYED_PILE_Y,
    },
};

use super::{BoardColor, SpawnCardExt};

pub fn setup_cards(mut commands: Commands) {
    for index in 0..CARDS_COUNT {
        commands.spawn_card(index, format!("Card {}", index));
    }
}

use std::ops::Deref;

use bevy::{core_pipeline::tonemapping::Tonemapping, ecs::system::SystemParam, prelude::*};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Tonemapping::BlenderFilmic,
        Transform::from_translation(CAMERA_POSITION).looking_at(CAMERA_TARGET, CAMERA_UP),
    ));
}

// Parameterized system without SystemParam, RETURNS the desired system
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

#[derive(SystemParam)]
pub struct BoardColorParam<'w>(pub Res<'w, BoardColor>);

// Optional, but prevents the need of triple dereferencing, e.g. `***board_color_param`
impl Deref for BoardColorParam<'_> {
    type Target = Color;

    fn deref(&self) -> &Self::Target {
        &self.0.0
    }
}

// Parameterized system with SystemParam, IS the desired system, the parameter is "injected" by Bevy
pub fn setup_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    board_color_param: BoardColorParam,
) {
    commands.spawn((
        Name::new("Board"),
        Mesh3d(meshes.add(Rectangle::from_length(BOARD_SIDE_LENGTH))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: *board_color_param,
            ..default()
        })),
    ));

    commands.spawn((
        DeckPile::<Player>::new(),
        Transform::from_xyz(PLAYER_DECK_PILE_X, PLAYER_DECK_PILE_Y, 0.)
            .with_rotation(*PLAYER_DECK_PILE_ROTATION),
    ));

    commands.spawn((
        GraveyardPile::<Player>::new(),
        Transform::from_xyz(PLAYER_GRAVEYARD_PILE_X, PLAYER_GRAVEYARD_PILE_Y, 0.)
            .with_rotation(*PLAYER_GRAVEYARD_PILE_ROTATION),
    ));

    commands.spawn((
        PlayedPile::<Player>::new(),
        Transform::from_xyz(PLAYER_PLAYED_PILE_X, PLAYER_PLAYED_PILE_Y, 0.)
            .with_rotation(*PLAYER_PLAYED_PILE_ROTATION),
    ));
}
