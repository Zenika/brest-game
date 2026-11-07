use bevy::prelude::*;
use shared::ContestantID;

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct PlayerID(pub ContestantID);

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct OpponentID(pub ContestantID);
