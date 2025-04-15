use bevy::prelude::*;

use crate::card_location::CardLocation;

pub fn on_turn_ending(mut query: Query<&mut CardLocation>) {
    query.iter_mut().for_each(|mut location| {
        if *location == CardLocation::Board || *location == CardLocation::Hand {
            *location = CardLocation::Graveyard;
        }
    });
}
