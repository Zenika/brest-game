use bevy::ecs::{
    event::Event,
    message::{MessageReader, MessageWriter},
};
use entity_message::EntityMessage;

use crate::playground::{LocatedEntityMessage, PlaygroundLocation};

pub fn locate_card_entity_message<E: Event + Clone, Location: PlaygroundLocation>(
    mut card_messages: MessageReader<EntityMessage<E>>,
    mut located_messages: MessageWriter<LocatedEntityMessage<E, Location>>,
) {
    for message in card_messages.read() {
        located_messages.write(LocatedEntityMessage::new(message.clone()));
    }
}
