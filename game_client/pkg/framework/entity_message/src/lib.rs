use std::marker::PhantomData;

use bevy::ecs::{
    entity::{ContainsEntity, Entity},
    event::EntityEvent,
    message::{Message, MessageWriter},
    observer::On,
};

#[derive(Message, Clone, Copy)]
pub struct EntityMessage<M> {
    entity: Entity,
    marker: PhantomData<M>,
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
            marker: PhantomData::<M>,
        }
    }
}

pub fn relay_as_entity_message<EE: EntityEvent>(
    trigger: On<EE>,
    mut message: MessageWriter<EntityMessage<EE>>,
) {
    message.write(trigger.event().event_target().into());
}
