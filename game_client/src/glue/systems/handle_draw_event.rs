use bevy::prelude::*;

use crate::{card_location::CardLocation, glue::events::DrawEvent};

pub fn handle_draw_event(
    mut draw_events: EventReader<DrawEvent>,
    mut query: Query<&mut CardLocation>,
) {
    for event in draw_events.read() {
        if let Ok(mut location) = query.get_mut(event.entity) {
            *location = CardLocation::Hand;
        }
    }
}
