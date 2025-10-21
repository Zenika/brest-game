use bevy::prelude::*;
use shared::{Play, PlayRequest};

pub fn on_play_request(
    mut messages_in: MessageReader<PlayRequest>,
    mut messages_out: MessageWriter<Play>,
    mut _commands: Commands,
) {
    for message_in in messages_in.read() {
        println!("MockServer\ton_play_request\tIN\t{:?}", message_in);
        let PlayRequest(contestant_id, card_id) = message_in;

        let message_out = Play(*contestant_id, *card_id);
        println!("MockServer\ton_play_request\tOUT\t{:?}", message_out);

        messages_out.write(message_out);
    }
}
