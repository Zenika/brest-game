use anima::enable_anima;
use bevy::prelude::*;
use shared::ContestantID;

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

type Hover = Pointer<Over>;

pub struct GluePlugin;

impl Plugin for GluePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(PlayerID(ContestantID(0)))
            .insert_resource(OpponentID(ContestantID(1)))
            .add_event::<DrawEvent>()
            .add_systems(PostStartup, enable_anima::<With<CardLocation>>)
            .add_systems(PostStartup, round::set_phase(&RoundPhase::Starting))
            .add_systems(OnEnter(RoundPhase::Starting), on_round_starting)
            .add_systems(
                Update,
                (
                    (
                        apply_hover_material_on::<Hover>(Some(CardLocation::Hand)),
                        request_player_play,
                    )
                        .run_if(in_state(PlayerPlayed(ContestantPlayed::No))),
                    request_opponent_play.run_if(in_state(OpponentPlayed(ContestantPlayed::No))),
                    check_for_playing_phase_done,
                )
                    .run_if(in_state(RoundPhase::Playing)),
            )
            .add_systems(
                Update,
                round::transition_on_timer_policy(&RoundPhase::Resolving, &RoundPhase::Ending),
            )
            .add_systems(OnEnter(RoundPhase::Ending), on_round_ending)
            .add_systems(
                Update,
                round::transition_on_timer_policy(&RoundPhase::Ending, &RoundPhase::Starting),
            )
            .add_systems(
                Update,
                (
                    apply_base_material_on::<Pointer<Out>>(None),
                    arrange_board,
                    arrange_deck,
                    arrange_hand,
                    arrange_graveyard,
                    handle_draw_event,
                    handle_play,
                ),
            )
            // Components
            .register_type::<PlayerID>()
            .register_type::<OpponentID>();
    }
}
