use anima::{Anima, WithTRS};
use bevy::prelude::*;

use crate::{
    board_locations::{GraveyardPile, Player},
    card_location::Graveyard,
    constants::CARD_THICKNESS,
    sequences::GraveyardSequenceStamp,
};

pub fn arrange_graveyard(
    mut cards_query: Query<(&Graveyard, &GraveyardSequenceStamp, &mut Anima)>,
    graveyard_pile_query: Query<(&GraveyardPile<Player>, &Transform)>,
) {
    let (_, graveyard_pile_transform) = graveyard_pile_query.single();

    let mut cards: Vec<_> = cards_query.iter_mut().collect();

    cards.sort_by(|(_, seq_stamp_a, _), (_, seq_stamp_b, _)| seq_stamp_a.cmp(seq_stamp_b));

    cards
        .into_iter()
        .enumerate()
        .for_each(|(index, (_, _, mut anima))| {
            let target_translation = Vec3::new(
                graveyard_pile_transform.translation.x,
                graveyard_pile_transform.translation.y,
                CARD_THICKNESS * (index + 1) as f32,
            );

            anima.set_if_neq(
                anima
                    .with_translation((target_translation, None, None))
                    .with_rotation((graveyard_pile_transform.rotation, None, None)),
            );
        });
}
