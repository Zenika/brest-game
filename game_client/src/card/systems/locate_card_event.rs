use bevy::prelude::*;

use crate::card::{
    components::CardLocation,
    events::{CardEvent, LocatedCardEvent},
};

pub fn locate_card_event<E: Event>(
    mut card_events: EventReader<CardEvent<E>>,
    mut query: Query<&CardLocation>,
    mut located_events: EventWriter<LocatedCardEvent<E>>,
) {
    for event in card_events.read() {
        if let Ok(location) = query.get_mut(event.entity()) {
            located_events.send(LocatedCardEvent::<E>::new(event.entity(), location.clone()));
        }
    }
}
