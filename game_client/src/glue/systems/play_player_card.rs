use bevy::prelude::*;

use crate::{
    glue::states::PlayerPlayed,
    card_location::{CardLocation, LocatedCardEvent},
};

pub fn play_player_card(
    mut events: EventReader<LocatedCardEvent<Pointer<Click>>>,
    mut query: Query<&mut CardLocation>,
    mut next_state: ResMut<NextState<PlayerPlayed>>,
) {
    if let Some(event) = events.read().last() {
        if let Ok(mut location) = query.get_mut(event.entity()) {
            if *event.location() == CardLocation::Hand {
                *location = CardLocation::Board;
                next_state.set(PlayerPlayed::Yes);
            }
        }
    }
}
