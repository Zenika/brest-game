use std::f32::consts::PI;

use anima::{Anima, WithTRS};
use bevy::prelude::*;

use crate::card::{
    components::{CardLocation, DeckSequenceStamp},
    constants::GAP,
};

pub fn arrange_deck(mut query: Query<(&CardLocation, &DeckSequenceStamp, &mut Anima)>) {
    let target_location = CardLocation::Deck;
    let target_rotation = Quat::from_rotation_y(-PI);

    let mut cards: Vec<_> = query
        .iter_mut()
        .filter(|&(location, _, _)| *location == target_location)
        .collect();

    cards.sort_by(|(_, seq_stamp_a, _), (_, seq_stamp_b, _)| seq_stamp_a.cmp(seq_stamp_b));

    cards
        .into_iter()
        .enumerate()
        .for_each(|(index, (_, _, mut anima))| {
            let target_translation = Vec3::new(4., -4., (index as f32 + 1.) * GAP);

            anima.set_if_neq(
                anima
                    .with_translation((target_translation, None, None))
                    .with_rotation((target_rotation, None, None)),
            );
        });
}
