use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum TurnState {
    #[default]
    Irrelevant,
    Starting,
    Playing,
    Resolving,
    Ending,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum PlayerPlayed {
    Yes,
    #[default]
    No,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum OpponentPlayed {
    Yes,
    #[default]
    No,
}
