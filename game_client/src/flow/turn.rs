use bevy::prelude::*;

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
#[component(immutable)]
pub struct Player;

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
#[component(immutable)]
pub struct Opponent;

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

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_state::<PlayerPlayed>()
            .register_type::<State<PlayerPlayed>>()
            .register_type::<NextState<PlayerPlayed>>();

        app.init_state::<OpponentPlayed>()
            .register_type::<State<OpponentPlayed>>()
            .register_type::<NextState<OpponentPlayed>>();
    }
}
