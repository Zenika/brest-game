use bevy::{ecs::query, prelude::*, render::render_phase::Draw};

use crate::card::{
    components::CardLocation,
    events::{DrawEvent, LocatedCardEvent},
};

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

pub fn draw_on_deck_click(
    mut card_events: EventReader<LocatedCardEvent<Pointer<Click>>>,
    mut draw_events: EventWriter<DrawEvent>,
) {
    for event in card_events.read() {
        if *event.location() == CardLocation::Deck {
            draw_events.send(DrawEvent {
                entity: event.entity(),
            });
        }
    }
}

pub fn update_location_on_draw(
    mut draw_events: EventReader<DrawEvent>,
    mut query: Query<&mut CardLocation>,
) {
    for event in draw_events.read() {
        if let Ok(mut location) = query.get_mut(event.entity) {
            *location = CardLocation::Hand;
        }
    }
}
