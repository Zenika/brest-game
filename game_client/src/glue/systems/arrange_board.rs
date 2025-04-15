use anima::{Anima, WithTED};
use bevy::prelude::*;

use crate::{
    card_location::CardLocation,
    constants::{CARD_THICKNESS, PLAYED_CARD_ROTATION, PLAYED_CARD_X, PLAYED_CARD_Y},
};

pub fn arrange_board(mut query: Query<(&CardLocation, &mut Anima), Changed<CardLocation>>) {
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
