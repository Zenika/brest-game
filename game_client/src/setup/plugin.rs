use bevy::prelude::*;

use super::setup_cards;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cards);
    }
}
