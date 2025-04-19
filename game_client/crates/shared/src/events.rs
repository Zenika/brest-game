use bevy::ecs::event::Event;

use crate::{CardID, ContestantID};

#[derive(Event, Debug)]
pub struct PlayRequest(pub ContestantID, pub CardID);

#[derive(Event, Debug)]
pub struct Play(pub ContestantID, pub CardID);
