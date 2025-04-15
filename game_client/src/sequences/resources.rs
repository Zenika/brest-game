use bevy::prelude::*;

#[derive(Resource, Deref, Reflect)]
#[reflect(Resource)]
pub struct DeckSequence(u8);

impl DeckSequence {
    pub fn next(&mut self) -> u8 {
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

impl HandSequence {
    pub fn next(&mut self) -> u8 {
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
pub struct GraveyardSequence(u8);

impl GraveyardSequence {
    pub fn next(&mut self) -> u8 {
        self.0 += 1;

        self.0
    }
}

impl FromWorld for GraveyardSequence {
    fn from_world(_: &mut World) -> Self {
        Self(0)
    }
}
