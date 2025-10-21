use bevy::prelude::*;
use shared::{CardID, ContestantID, Play, PlayRequest};

use crate::{
    area::{Hand, LocatedCardMessage, Played},
    glue::resources::{OpponentID, PlayerID},
    turn::{ContestantPlayed, OpponentPlayed, PlayerPlayed},
};

pub fn request_player_play(
    mut messages_in: MessageReader<LocatedCardMessage<Pointer<Click>, Hand>>,
    mut query: Query<&CardID>,
    mut messages_out: MessageWriter<PlayRequest>,
) {
    if let Some(message) = messages_in.read().last()
        && let Ok(card_id) = query.get_mut(message.entity())
    {
        messages_out.write(PlayRequest(ContestantID(0), *card_id));
    }
}

pub fn request_opponent_play(
    keys: Res<ButtonInput<KeyCode>>,
    mut messages_out: MessageWriter<PlayRequest>,
) {
    if keys.just_pressed(KeyCode::Space) {
        messages_out.write(PlayRequest(ContestantID(1), CardID(255))); // Fake card ID
    }
}

pub fn handle_play(
    mut commands: Commands,
    player_id: Res<PlayerID>,
    opponent_id: Res<OpponentID>,
    mut messages_in: MessageReader<Play>,
    mut player_played_next_state: ResMut<NextState<PlayerPlayed>>,
    mut opponent_played_next_state: ResMut<NextState<OpponentPlayed>>,
    mut query: Query<(Entity, &CardID), With<Hand>>,
) {
    let PlayerID(player_contestant_id) = *player_id;
    let OpponentID(opponent_contestant_id) = *opponent_id;

    for &Play(contestant_id, played_card_id) in messages_in.read() {
        if contestant_id == player_contestant_id {
            player_played_next_state.set(PlayerPlayed(ContestantPlayed::Yes));
        }

        if contestant_id == opponent_contestant_id {
            opponent_played_next_state.set(OpponentPlayed(ContestantPlayed::Yes));
        }

        for (entity, &card_id) in query.iter_mut() {
            if card_id == played_card_id {
                commands.entity(entity).insert(Played);
            }
        }
    }
}
