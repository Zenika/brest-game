use std::f32::consts::PI;

use anima::{Anima, WithTRS};
use bevy::prelude::*;

use crate::card::{
    components::{CardLocation, HandSequenceStamp},
    constants::{CARD_SIZE, GAP},
};

pub fn arrange_hand(mut query: Query<(&CardLocation, &HandSequenceStamp, &mut Anima)>) {
    let target_location = CardLocation::Hand;
    let target_rotation = Quat::from_rotation_x(PI / 4.);
    let x_step = CARD_SIZE.x + GAP;

    let mut cards: Vec<_> = query
        .iter_mut()
        .filter(|&(location, _, _)| *location == target_location)
        .collect();

    let count = cards.len() as f32;

    cards.sort_by(|(_, seq_stamp_a, _), (_, seq_stamp_b, _)| seq_stamp_a.cmp(seq_stamp_b));

    cards
        .into_iter()
        .enumerate()
        .for_each(|(index, (_, _, mut anima))| {
            let target_translation = Vec3::new(
                (-x_step * (count - 1.) / 2.) + (index as f32 * x_step),
                -5.,
                2.,
            );

            anima.set_if_neq(
                anima
                    .with_translation((target_translation, None, None))
                    .with_rotation((target_rotation, None, None)),
            );
        });
}
