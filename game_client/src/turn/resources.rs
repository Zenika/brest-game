use bevy::prelude::*;

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct TurnPhaseTimer(Timer);

impl TurnPhaseTimer {
    pub fn new(duration_in_seconds: f32) -> Self {
        Self(Timer::from_seconds(duration_in_seconds, TimerMode::Once))
    }
}
