use bevy::prelude::*;

use super::{CardEvent, CardLocation, LocatedCardEvent};

pub fn locate_card_event<E: Event, Location: Component + CardLocation>(
    mut card_events: EventReader<CardEvent<E>>,
    mut located_events: EventWriter<LocatedCardEvent<E, Location>>,
) {
    for event in card_events.read() {
        located_events.write(LocatedCardEvent::new(event.entity()));
    }
}
