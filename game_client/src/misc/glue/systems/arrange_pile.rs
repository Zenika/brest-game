use anima::{Anima, WithTED};
use bevy::prelude::*;

use crate::misc::constants::CARD_THICKNESS;
use crate::playground::{PlaygroundLocationTag, SequenceStamp};

pub fn arrange_pile<C: Component, A: Component>(
    mut cards_query: Query<(&SequenceStamp<A>, &mut Anima), (With<C>, With<A>)>,
    playground_part_query: Single<&Transform, (With<PlaygroundLocationTag>, With<C>, With<A>)>,
) {
    let mut cards: Vec<_> = cards_query.iter_mut().collect();

    cards.sort_by(|(seq_stamp_a, _), (seq_stamp_b, _)| seq_stamp_a.cmp(seq_stamp_b));

    cards.into_iter().enumerate().for_each(|(index, (_, mut anima))| {
        let target_translation = Vec3::new(
            playground_part_query.translation.x,
            playground_part_query.translation.y,
            CARD_THICKNESS * (index + 1) as f32,
        );

        // TODO: test Anima / Parenting compat
        anima.set_if_neq(anima.clone().with_transforms((target_translation, playground_part_query.rotation, None)));
    });
}
