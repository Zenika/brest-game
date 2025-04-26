use anima::{Anima, WithTRS};
use bevy::prelude::*;

use crate::{
    board_locations::{DeckPile, Player},
    card_location::Deck,
    constants::CARD_THICKNESS,
    sequences::DeckSequenceStamp,
};

pub fn arrange_deck(
    mut cards_query: Query<(&Deck, &DeckSequenceStamp, &mut Anima)>,
    deck_pile_query: Query<(&DeckPile<Player>, &Transform)>,
) -> Result {
    let (_, deck_pile_transform) = deck_pile_query.single()?;

    let mut cards: Vec<_> = cards_query.iter_mut().collect();

    cards.sort_by(|(_, seq_stamp_a, _), (_, seq_stamp_b, _)| seq_stamp_a.cmp(seq_stamp_b));

    cards
        .into_iter()
        .enumerate()
        .for_each(|(index, (_, _, mut anima))| {
            let target_translation = Vec3::new(
                deck_pile_transform.translation.x,
                deck_pile_transform.translation.y,
                CARD_THICKNESS * (index + 1) as f32,
            );

            // TODO: test Anima / Parenting compat
            anima.set_if_neq(
                anima
                    .with_translation((target_translation, None, None))
                    .with_rotation((deck_pile_transform.rotation, None, None)),
            );
        });

    Ok(())
}
