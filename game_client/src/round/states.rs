use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum RoundPhase {
    #[default]
    Waiting,
    Starting,
    Playing,
    Resolving,
    Ending,
}
