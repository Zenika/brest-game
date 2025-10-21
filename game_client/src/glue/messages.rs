use bevy::ecs::{entity::Entity, message::Message};

#[derive(Message)]
pub struct DrawMessage {
    pub entity: Entity,
}
