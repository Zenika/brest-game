use bevy::prelude::*;

use crate::card::{components::CardLocation, events::LocatedCardEvent, resources::CardMaterial};

pub fn apply_material_on<CM: CardMaterial + Resource, E: Event>(
    location_filter: Option<CardLocation>,
) -> impl Fn(EventReader<LocatedCardEvent<E>>, Res<CM>, Query<&mut MeshMaterial3d<StandardMaterial>>)
{
    move |mut events, card_material, mut query| {
        for event in events.read() {
            if let Ok(mut material) = query.get_mut(event.entity()) {
                match location_filter {
                    None => material.0 = card_material.as_material(),

                    Some(ref filter) if *event.location() == *filter => {
                        material.0 = card_material.as_material();
                    }

                    _ => continue,
                };
            }
        }
    }
}
