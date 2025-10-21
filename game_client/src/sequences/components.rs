use std::marker::PhantomData;

use bevy::prelude::*;

#[derive(Component, Reflect, PartialEq, Eq, PartialOrd, Ord, Deref)]
#[reflect(Component)]
pub struct SequenceStamp<T> {
    #[deref]
    value: u8,
    marker: PhantomData<T>,
}

impl<T: Send + Sync> From<u8> for SequenceStamp<T> {
    fn from(value: u8) -> Self {
        Self {
            value,
            marker: PhantomData::<T>,
        }
    }
}
