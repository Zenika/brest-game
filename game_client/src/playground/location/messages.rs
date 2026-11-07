use std::marker::PhantomData;

use bevy::ecs::entity::{ContainsEntity, Entity};
use bevy::ecs::message::Message;
use entity_message::EntityMessage;

use super::PlaygroundLocation;

#[derive(Message)]
pub struct LocatedEntityMessage<M, Location: PlaygroundLocation> {
    entity_message: EntityMessage<M>,
    location: PhantomData<Location>,
}

impl<M, Location: PlaygroundLocation> ContainsEntity for LocatedEntityMessage<M, Location> {
    fn entity(&self) -> Entity {
        self.entity_message.entity()
    }
}

impl<M, Location: PlaygroundLocation> LocatedEntityMessage<M, Location> {
    pub fn new(entity_message: EntityMessage<M>) -> Self {
        Self { entity_message, location: PhantomData::<Location> }
    }
}
