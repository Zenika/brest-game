use bevy::prelude::*;
use shared::{Play, PlayRequest};

pub fn on_play_request(
    mut events_in: EventReader<PlayRequest>,
    mut events_out: EventWriter<Play>,
    mut _commands: Commands,
) {
    for event_in in events_in.read() {
        println!("MockServer\ton_play_request\tIN\t{:?}", event_in);
        let PlayRequest(contestant_id, card_id) = event_in;

        let event_out = Play(*contestant_id, *card_id);
        println!("MockServer\ton_play_request\tOUT\t{:?}", event_out);

        events_out.write(event_out);
    }
}
