use std::marker::PhantomData;

use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Opponent;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DeckPile<T>(PhantomData<T>);

impl<T> DeckPile<T> {
    pub fn new() -> Self {
        Self(PhantomData::<T>)
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GraveyardPile<T>(PhantomData<T>);

impl<T> GraveyardPile<T> {
    pub fn new() -> Self {
        Self(PhantomData::<T>)
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayedPile<T>(PhantomData<T>);

impl<T> PlayedPile<T> {
    pub fn new() -> Self {
        Self(PhantomData::<T>)
    }
}
