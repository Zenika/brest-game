use bevy::ecs::message::Message;

use crate::{CardID, ContestantID};

#[derive(Message, Debug)]
pub struct PlayRequest(pub ContestantID, pub CardID);

#[derive(Message, Debug)]
pub struct Play(pub ContestantID, pub CardID);
