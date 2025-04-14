use anima::{Anima, WithTRS};
use bevy::prelude::*;

use crate::{
    card_location::CardLocation,
    constants::{CARD_THICKNESS, GRAVEYARD_CARD_ROTATION, GRAVEYARD_CARD_X, GRAVEYARD_CARD_Y},
    sequences::GraveyardSequenceStamp,
};

pub fn arrange_graveyard(mut query: Query<(&CardLocation, &GraveyardSequenceStamp, &mut Anima)>) {
    let target_location = CardLocation::Graveyard;
    let target_rotation = *GRAVEYARD_CARD_ROTATION;

    let mut cards: Vec<_> = query
        .iter_mut()
        .filter(|&(location, _, _)| *location == target_location)
        .collect();

    cards.sort_by(|(_, seq_stamp_a, _), (_, seq_stamp_b, _)| seq_stamp_a.cmp(seq_stamp_b));

    cards
        .into_iter()
        .enumerate()
        .for_each(|(index, (_, _, mut anima))| {
            let target_translation = Vec3::new(
                GRAVEYARD_CARD_X,
                GRAVEYARD_CARD_Y,
                CARD_THICKNESS * (index + 1) as f32,
            );

            anima.set_if_neq(
                anima
                    .with_translation((target_translation, None, None))
                    .with_rotation((target_rotation, None, None)),
            );
        });
}
