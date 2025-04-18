use anima::enable_anima;
use bevy::prelude::*;
use shared::{ContestantID, Play};

use crate::{
    card_location::CardLocation,
    card_material::{apply_base_material_on, apply_hover_material_on},
    round::{self, RoundPhase},
    turn::{ContestantPlayed, OpponentPlayed, PlayerPlayed},
};

use super::{
    events::*,
    resources::{OpponentID, PlayerID},
    systems::*,
};

pub struct GluePlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum PlayerPlayedSet {
    No,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum OpponentPlayedSet {
    No,
}

impl Plugin for GluePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.configure_sets(
            Update,
            (
                RoundPhase::Playing.run_if(in_state(RoundPhase::Playing)), // The RoundPhase enum is a States AND a SystemSet
                PlayerPlayedSet::No.run_if(in_state(PlayerPlayed(ContestantPlayed::No))),
                OpponentPlayedSet::No.run_if(in_state(OpponentPlayed(ContestantPlayed::No))),
            ),
        );

        app.insert_resource(PlayerID(ContestantID(0)))
            .insert_resource(OpponentID(ContestantID(1)))
            .add_event::<DrawEvent>()
            .add_systems(PostStartup, enable_anima::<With<CardLocation>>)
            .add_systems(OnEnter(RoundPhase::Starting), on_round_starting)
            .add_systems(
                Update,
                (
                    apply_hover_material_on::<Pointer<Over>>(Some(CardLocation::Hand))
                        .in_set(PlayerPlayedSet::No),
                    request_player_play.in_set(PlayerPlayedSet::No),
                    request_opponent_play.in_set(OpponentPlayedSet::No),
                    end_playing_phase_policy,
                )
                    .in_set(RoundPhase::Playing),
            )
            .add_systems(
                Update,
                round::transition_on_timer_policy(&RoundPhase::Resolving, &RoundPhase::Ending),
            )
            .add_systems(OnEnter(RoundPhase::Ending), on_round_ending)
            .add_systems(
                Update,
                (
                    apply_base_material_on::<Pointer<Out>>(None),
                    arrange_board,
                    arrange_deck,
                    arrange_hand,
                    arrange_graveyard,
                    handle_draw_event,
                    handle_play.run_if(on_event::<Play>),
                ),
            )
            // Components
            .register_type::<PlayerID>()
            .register_type::<OpponentID>();
    }
}
