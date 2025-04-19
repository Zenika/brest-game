use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum ContestantPlayed {
    #[default]
    No,
    Yes,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub struct OpponentPlayed(pub ContestantPlayed);

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub struct PlayerPlayed(pub ContestantPlayed);
