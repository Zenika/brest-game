use bevy::prelude::*;

use crate::card::{components::CardLocation, events::LocatedCardEvent};

pub fn translocate_on<E: Event>(
    from: CardLocation,
    to: CardLocation,
) -> impl Fn(EventReader<LocatedCardEvent<E>>, Query<&mut CardLocation>) {
    move |mut events, mut query| {
        for event in events.read() {
            if let Ok(mut location) = query.get_mut(event.entity()) {
                if *event.location() == from {
                    *location = to.clone();
                }
            }
        }
    }
}
