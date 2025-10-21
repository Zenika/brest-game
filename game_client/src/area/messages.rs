use std::marker::PhantomData;

use bevy::ecs::{entity::Entity, message::Message};
use entity_message::EntityMessage;

pub type CardMessage<E> = EntityMessage<E>;

use super::Area;

#[derive(Message)]
pub struct LocatedCardMessage<M, Location: Area> {
    entity: Entity,
    location_marker: PhantomData<Location>,
    message_marker: PhantomData<CardMessage<M>>,
}

impl<M, Location: Area> LocatedCardMessage<M, Location> {
    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            location_marker: PhantomData::<Location>,
            message_marker: PhantomData::<CardMessage<M>>,
        }
    }
}
