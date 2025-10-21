use std::marker::PhantomData;

use bevy::prelude::*;

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct StatesTimer<S: States>(#[deref] Timer, #[reflect(ignore)] PhantomData<S>);

impl<S: States> StatesTimer<S> {
    pub fn new(duration_in_seconds: f32) -> Self {
        Self(
            Timer::from_seconds(duration_in_seconds, TimerMode::Once),
            PhantomData::<S>,
        )
    }
}
