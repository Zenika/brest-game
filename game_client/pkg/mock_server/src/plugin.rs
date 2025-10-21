use bevy::prelude::*;
use shared::{Play, PlayRequest};

use crate::{OPPONENT_CONTESTANT_ID, PLAYER_CONTESTANT_ID, ServerState, on_play_request};

pub struct MockServer;

impl Plugin for MockServer {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayRequest>()
            .add_event::<Play>()
            .add_systems(Update, on_play_request.run_if(on_event::<PlayRequest>))
            .insert_resource(ServerState {
                player_contestant_id: PLAYER_CONTESTANT_ID,
                opponent_contestant_id: OPPONENT_CONTESTANT_ID,
            })
            .register_type::<ServerState>();
    }
}
