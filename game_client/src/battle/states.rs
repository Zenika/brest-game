use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum BattlePhase {
    Started,
    #[default]
    InProgress,
    Ended,
    Resolved,
}
