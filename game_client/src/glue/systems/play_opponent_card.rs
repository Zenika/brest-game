use bevy::prelude::*;

use crate::glue::states::OpponentPlayed;

pub fn play_opponent_card(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<OpponentPlayed>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(OpponentPlayed::Yes);
    }
}
