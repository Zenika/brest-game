use bevy::prelude::*;

use crate::{area::Hand, glue::messages::DrawMessage};

pub fn handle_draw_message(mut commands: Commands, mut draw_messages: MessageReader<DrawMessage>) {
    for message in draw_messages.read() {
        commands.entity(message.entity).insert(Hand);
    }
}
