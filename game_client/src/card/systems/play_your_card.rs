use bevy::prelude::*;

use crate::card::{
    components::CardLocation,
    events::LocatedCardEvent,
    states::{OpponentPlayed, PlayerPlayed, TurnState},
};

pub fn play_your_card(
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

pub fn play_opponent_card(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<OpponentPlayed>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(OpponentPlayed::Yes);
    }
}

pub fn trigger_resolve(
    player_played: Res<State<PlayerPlayed>>,
    opponent_played: Res<State<OpponentPlayed>>,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    if *player_played == PlayerPlayed::Yes && *opponent_played == OpponentPlayed::Yes {
        next_state.set(TurnState::Resolving);
    }
}
