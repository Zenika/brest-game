use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Reflect)]
pub enum BattlePhase {
    #[default]
    Started,
    InProgress,
    Ended,
    Resolved,
}
