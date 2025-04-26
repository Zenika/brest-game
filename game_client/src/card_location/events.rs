use std::marker::PhantomData;

use bevy::ecs::{entity::Entity, event::Event};
use entity_event::EntityEvent;

pub type CardEvent<E> = EntityEvent<E>;

use super::CardLocation;

#[derive(Event)]
pub struct LocatedCardEvent<E, Location: CardLocation> {
    entity: Entity,
    location_marker: PhantomData<Location>,
    event_marker: PhantomData<CardEvent<E>>,
}

impl<E, Location: CardLocation> LocatedCardEvent<E, Location> {
    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            location_marker: PhantomData::<Location>,
            event_marker: PhantomData::<CardEvent<E>>,
        }
    }
}
