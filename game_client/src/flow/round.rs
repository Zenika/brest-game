use bevy::prelude::*;
use states_timer::{StatesTimer, reset_timer_on_transition};

#[derive(States, SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Reflect)]
pub enum RoundPhase {
    #[default]
    Waiting,
    Starting,
    Playing,
    Resolving,
    Ending,
}

pub struct RoundPlugin;

impl Plugin for RoundPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_state::<RoundPhase>().register_type::<State<RoundPhase>>().register_type::<NextState<RoundPhase>>();

        app.insert_resource(StatesTimer::<RoundPhase>::new(1.0)).register_type::<StatesTimer<RoundPhase>>();

        app.add_systems(Update, reset_timer_on_transition::<RoundPhase>());
    }
}
