use bevy::prelude::*;

use crate::{
    card_location::{Deck, Hand},
    glue::events::DrawEvent,
};

pub fn handle_draw_event(mut commands: Commands, mut draw_events: EventReader<DrawEvent>) {
    for event in draw_events.read() {
        commands.entity(event.entity).remove::<Deck>().insert(Hand);
    }
}
