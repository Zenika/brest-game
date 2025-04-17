use bevy::prelude::*;
use shared::{CardID, ContestantID, Play, PlayRequest};

use crate::{
    card_location::{CardLocation, LocatedCardEvent},
    glue::resources::{OpponentID, PlayerID},
    turn::{ContestantPlayed, OpponentPlayed, PlayerPlayed},
};

pub fn request_player_play(
    mut events_in: EventReader<LocatedCardEvent<Pointer<Click>>>,
    mut query: Query<&CardID>,
    mut events_out: EventWriter<PlayRequest>,
) {
    if let Some(event) = events_in.read().last() {
        if let Ok(card_id) = query.get_mut(event.entity()) {
            if *event.location() == CardLocation::Hand {
                events_out.send(PlayRequest(ContestantID(0), *card_id));
            }
        }
    }
}

pub fn request_opponent_play(
    keys: Res<ButtonInput<KeyCode>>,
    mut events_out: EventWriter<PlayRequest>,
) {
    if keys.just_pressed(KeyCode::Space) {
        events_out.send(PlayRequest(ContestantID(1), CardID(255))); // Fake card ID
    }
}

pub fn handle_play(
    player_id: Res<PlayerID>,
    opponent_id: Res<OpponentID>,
    mut events_in: EventReader<Play>,
    mut player_played_next_state: ResMut<NextState<PlayerPlayed>>,
    mut opponent_played_next_state: ResMut<NextState<OpponentPlayed>>,
    mut query: Query<(&CardID, &mut CardLocation)>,
) {
    let PlayerID(player_contestant_id) = *player_id;
    let OpponentID(opponent_contestant_id) = *opponent_id;

    for &Play(contestant_id, played_card_id) in events_in.read() {
        if contestant_id == player_contestant_id {
            player_played_next_state.set(PlayerPlayed(ContestantPlayed::Yes));
        }

        if contestant_id == opponent_contestant_id {
            opponent_played_next_state.set(OpponentPlayed(ContestantPlayed::Yes));
        }

        for (&card_id, mut location) in query.iter_mut() {
            if card_id == played_card_id {
                *location = CardLocation::Board;
            }
        }
    }
}
