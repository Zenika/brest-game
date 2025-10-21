use std::marker::PhantomData;

use bevy::ecs::{
    entity::{ContainsEntity, Entity},
    event::EntityEvent,
    message::{Message, MessageWriter},
    observer::On,
};

#[derive(Message)]
pub struct EntityMessage<M> {
    entity: Entity,
    message_marker: PhantomData<M>,
}

impl<M> ContainsEntity for EntityMessage<M> {
    fn entity(&self) -> Entity {
        self.entity
    }
}

impl<M> From<Entity> for EntityMessage<M> {
    fn from(value: Entity) -> Self {
        Self {
            entity: value,
            message_marker: PhantomData::<M>,
        }
    }
}

pub fn send_entity_message_on<EE: EntityEvent, EM: Message + From<Entity>>(
    trigger: On<EE>,
    mut message: MessageWriter<EM>,
) {
    message.write(trigger.event().event_target().into());
}
