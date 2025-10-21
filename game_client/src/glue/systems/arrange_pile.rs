use anima::{Anima, WithTRS};
use bevy::prelude::*;

use crate::{area::PlaygroundPart, constants::CARD_THICKNESS, sequences::SequenceStamp};

pub fn arrange_pile<C: Component, A: Component>(
    mut cards_query: Query<(&SequenceStamp<A>, &mut Anima), (With<C>, With<A>)>,
    playground_part_query: Single<&Transform, (With<PlaygroundPart>, With<C>, With<A>)>,
) {
    let mut cards: Vec<_> = cards_query.iter_mut().collect();

    cards.sort_by(|(seq_stamp_a, _), (seq_stamp_b, _)| seq_stamp_a.cmp(seq_stamp_b));

    cards
        .into_iter()
        .enumerate()
        .for_each(|(index, (_, mut anima))| {
            let target_translation = Vec3::new(
                playground_part_query.translation.x,
                playground_part_query.translation.y,
                CARD_THICKNESS * (index + 1) as f32,
            );

            // TODO: test Anima / Parenting compat
            anima.set_if_neq(
                anima
                    .with_translation((target_translation, None, None))
                    .with_rotation((playground_part_query.rotation, None, None)),
            );
        });
}
