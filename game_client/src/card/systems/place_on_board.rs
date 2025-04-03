use anima::{Anima, WithTED};
use bevy::prelude::*;

use crate::card::{components::CardLocation, constants::GAP};

pub fn place_on_board(mut query: Query<(&CardLocation, &mut Anima), Changed<CardLocation>>) {
    for (card_location, mut anima) in &mut query {
        if let CardLocation::Board = card_location {
            anima.set_if_neq(anima.with_transform((
                Vec3::new(0., -2., GAP),
                Quat::from_rotation_x(0.),
                None,
            )));
        }
    }
}
