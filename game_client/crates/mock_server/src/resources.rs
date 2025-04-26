use bevy::prelude::*;
use shared::ContestantID;

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct ServerState {
    pub player_contestant_id: ContestantID,
    pub opponent_contestant_id: ContestantID,
}
