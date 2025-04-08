use anima::{Anima, WithTED};
use bevy::prelude::*;

use crate::card::{
    components::CardLocation,
    constants::{CARD_THICKNESS, PLAYED_CARD_ROTATION, PLAYED_CARD_X, PLAYED_CARD_Y},
};

pub fn place_on_board(mut query: Query<(&CardLocation, &mut Anima), Changed<CardLocation>>) {
    for (card_location, mut anima) in &mut query {
        if let CardLocation::Board = card_location {
            anima.set_if_neq(anima.with_transform((
                Vec3::new(PLAYED_CARD_X, PLAYED_CARD_Y, CARD_THICKNESS),
                *PLAYED_CARD_ROTATION,
                None,
            )));
        }
    }
}
