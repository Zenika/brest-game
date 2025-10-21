use anima::{Anima, WithTRS};
use bevy::prelude::*;

use crate::{
    constants::{CARD_SIZE, GAP, HAND_CARD_ROTATION, HAND_CARD_Y, HAND_CARD_Z},
    sequences::SequenceStamp,
};

pub fn arrange_fan<C: Component, A: Component>(
    mut query: Query<(&SequenceStamp<A>, &mut Anima), (With<C>, With<A>)>,
) {
    let target_rotation = *HAND_CARD_ROTATION;
    let x_step = CARD_SIZE.x + GAP;

    let mut cards: Vec<_> = query.iter_mut().collect();

    let count = cards.len();

    cards.sort_by(|(seq_stamp_a, _), (seq_stamp_b, _)| seq_stamp_a.cmp(seq_stamp_b));

    cards
        .into_iter()
        .enumerate()
        .for_each(|(index, (_, mut anima))| {
            let target_translation = Vec3::new(
                (-x_step / 2. * (count - 1) as f32) + (x_step * index as f32),
                HAND_CARD_Y,
                HAND_CARD_Z,
            );

            anima.set_if_neq(
                anima
                    .with_translation((target_translation, None, None))
                    .with_rotation((target_rotation, None, None)),
            );
        });
}
