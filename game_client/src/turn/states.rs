use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum TurnPhase {
    #[default]
    Waiting,
    Starting,
    Playing,
    Resolving,
    Ending,
}
