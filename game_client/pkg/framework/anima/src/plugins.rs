use bevy::prelude::*;

use super::animate_anima;

pub struct AnimaPlugin;

impl Plugin for AnimaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_anima);
    }
}
