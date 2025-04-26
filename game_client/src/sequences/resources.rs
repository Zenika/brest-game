use bevy::prelude::*;

pub trait Sequence: Resource + Reflect {
    fn next(&mut self) -> u8;
}

#[derive(Resource, Deref, Reflect)]
#[reflect(Resource)]
pub struct DeckSequence(u8);

impl Sequence for DeckSequence {
    fn next(&mut self) -> u8 {
        self.0 += 1;

        self.0
    }
}

impl FromWorld for DeckSequence {
    fn from_world(_: &mut World) -> Self {
        Self(0)
    }
}

#[derive(Resource, Deref, Reflect)]
#[reflect(Resource)]
pub struct HandSequence(u8);

impl Sequence for HandSequence {
    fn next(&mut self) -> u8 {
        self.0 += 1;

        self.0
    }
}

impl FromWorld for HandSequence {
    fn from_world(_: &mut World) -> Self {
        Self(0)
    }
}

#[derive(Resource, Deref, Reflect)]
#[reflect(Resource)]
pub struct PlayedSequence(u8);

impl Sequence for PlayedSequence {
    fn next(&mut self) -> u8 {
        self.0 += 1;

        self.0
    }
}

impl FromWorld for PlayedSequence {
    fn from_world(_: &mut World) -> Self {
        Self(0)
    }
}

#[derive(Resource, Deref, Reflect)]
#[reflect(Resource)]
pub struct GraveyardSequence(u8);

impl Sequence for GraveyardSequence {
    fn next(&mut self) -> u8 {
        self.0 += 1;

        self.0
    }
}

impl FromWorld for GraveyardSequence {
    fn from_world(_: &mut World) -> Self {
        Self(0)
    }
}
