use std::marker::PhantomData;

use bevy::ecs::{entity::Entity, event::Event};
use entity_event::EntityEvent;

pub type CardEvent<E> = EntityEvent<E>;

use super::CardLocation;

#[derive(Event)]
pub struct LocatedCardEvent<E> {
    entity: Entity,
    location: CardLocation,
    event_marker: PhantomData<CardEvent<E>>,
}

impl<E> LocatedCardEvent<E> {
    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn location(&self) -> &CardLocation {
        &self.location
    }
}

impl<Ev: Event> LocatedCardEvent<Ev> {
    pub fn new(entity: Entity, location: CardLocation) -> Self {
        Self {
            entity,
            location,
            event_marker: PhantomData,
        }
    }
}
