use bevy::prelude::*;

use crate::card_location::{CardLocation, LocatedCardEvent};

use super::resources::CardMaterial;

pub type Reader<'w, 's, E, L> = EventReader<'w, 's, LocatedCardEvent<E, L>>;

pub fn apply_material_on<CM: CardMaterial + Resource, E: Event, Location: CardLocation>(
    mut events: Reader<E, Location>,
    card_material: Res<CM>,
    mut query: Query<(&Location, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    for event in events.read() {
        if let Ok((_, mut material)) = query.get_mut(event.entity()) {
            material.0 = card_material.as_material();
        }
    }
}
