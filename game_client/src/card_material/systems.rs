use bevy::prelude::*;

use crate::card_location::CardLocation;

use super::{
    BaseCardMaterial, HoverCardMaterial,
    private::{Reader, apply_material_on},
};

pub fn apply_base_material_on<E: Event>(
    location_filter: Option<CardLocation>,
) -> impl Fn(Reader<E>, Res<BaseCardMaterial>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    apply_material_on::<BaseCardMaterial, E>(location_filter)
}

pub fn apply_hover_material_on<E: Event>(
    location_filter: Option<CardLocation>,
) -> impl Fn(Reader<E>, Res<HoverCardMaterial>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    apply_material_on::<HoverCardMaterial, E>(location_filter)
}
