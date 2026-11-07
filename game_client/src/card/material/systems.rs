use bevy::prelude::*;

use super::resources::CardMaterial;
use crate::playground::{LocatedEntityMessage, PlaygroundLocation};

pub type Reader<'w, 's, E, L> = MessageReader<'w, 's, LocatedEntityMessage<E, L>>;

pub fn apply_material_on<CM: CardMaterial + Resource, E: Message, Location: PlaygroundLocation>(
    mut messages: Reader<E, Location>,
    card_material: Res<CM>,
    mut query: Query<(&Location, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    for message in messages.read() {
        if let Ok((_, mut material)) = query.get_mut(message.entity()) {
            material.0 = card_material.as_material();
        }
    }
}
