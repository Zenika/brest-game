use anima::{Anima, WithTED};
use bevy::prelude::*;

use crate::{
    area::{Played, Player, PlaygroundArea},
    constants::CARD_THICKNESS,
};

pub fn arrange_board(
    mut query: Query<&mut Anima, With<Played>>,
    played_pile_query: Query<&Transform, (With<PlaygroundArea>, With<Played>, With<Player>)>,
) -> Result {
    let played_pile_transform = played_pile_query.single()?;

    for mut anima in &mut query {
        anima.set_if_neq(anima.with_transform((
            Vec3::new(
                played_pile_transform.translation.x,
                played_pile_transform.translation.y,
                CARD_THICKNESS,
            ),
            played_pile_transform.rotation,
            None,
        )));
    }

    Ok(())
}
