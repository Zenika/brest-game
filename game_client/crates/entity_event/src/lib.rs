use std::marker::PhantomData;

use bevy::ecs::{
    entity::Entity,
    event::{Event, EventWriter},
    observer::Trigger,
};

#[derive(Event)]
pub struct EntityEvent<Ev> {
    pub entity: Entity,
    event_marker: PhantomData<Ev>,
}

impl<Ev> From<Entity> for EntityEvent<Ev> {
    fn from(value: Entity) -> Self {
        Self {
            entity: value,
            event_marker: PhantomData::<Ev>,
        }
    }
}

pub fn send_entity_event_on<TE: Event, EE: Event + From<Entity>>(
    trigger: Trigger<TE>,
    mut event: EventWriter<EE>,
) {
    event.send(trigger.entity().into());
}
