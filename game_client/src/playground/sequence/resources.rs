use std::marker::PhantomData;

use bevy::prelude::*;

#[derive(Resource, Deref, Reflect)]
#[reflect(Resource)]
pub struct Sequence<T: Send + Sync> {
    #[deref]
    value: u8,
    marker: PhantomData<T>,
}

impl<T: Send + Sync> Sequence<T> {
    pub fn next(&mut self) -> u8 {
        self.value += 1;

        self.value
    }
}

impl<T: Send + Sync> FromWorld for Sequence<T> {
    fn from_world(_: &mut World) -> Self {
        Self {
            value: 0,
            marker: PhantomData::<T>,
        }
    }
}
