use bevy::ecs::entity::Entity;
use bevy::ecs::message::Message;

#[derive(Message)]
pub struct DrawMessage {
    pub entity: Entity,
}
