use std::marker::PhantomData;

use bevy::ecs::{
    entity::{ContainsEntity, Entity},
    event::{Event, EventWriter},
    observer::Trigger,
};

#[derive(Event)]
pub struct EntityEvent<E> {
    entity: Entity,
    event_marker: PhantomData<E>,
}

impl<E> ContainsEntity for EntityEvent<E> {
    fn entity(&self) -> Entity {
        self.entity
    }
}

impl<E> From<Entity> for EntityEvent<E> {
    fn from(value: Entity) -> Self {
        Self {
            entity: value,
            event_marker: PhantomData::<E>,
        }
    }
}

pub fn send_entity_event_on<TE: Event, EE: Event + From<Entity>>(
    trigger: Trigger<TE>,
    mut event: EventWriter<EE>,
) {
    event.write(trigger.target().into());
}
