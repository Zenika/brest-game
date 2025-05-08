use anima::{Anima, WithTED};
use bevy::prelude::*;

use crate::{
    area::{Played, Player, PlaygroundPart},
    constants::CARD_THICKNESS,
};

pub fn arrange_board(
    mut query: Query<&mut Anima, With<Played>>,
    played_pile_query: Single<&Transform, (With<PlaygroundPart>, With<Played>, With<Player>)>,
) {
    for mut anima in &mut query {
        anima.set_if_neq(anima.with_transform((
            Vec3::new(
                played_pile_query.translation.x,
                played_pile_query.translation.y,
                CARD_THICKNESS,
            ),
            played_pile_query.rotation,
            None,
        )));
    }
}
