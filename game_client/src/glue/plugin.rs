use anima::enable_anima;
use bevy::prelude::*;

use crate::{
    card_location::CardLocation,
    card_material::{apply_base_material_on, apply_hover_material_on},
    turn::{TurnPhase, set_turn_phase, transition_on_timer},
};

use super::{
    events::*,
    states::{OpponentPlayed, PlayerPlayed},
    systems::*,
};

type Hover = Pointer<Over>;

pub struct GluePlugin;

impl Plugin for GluePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_state::<PlayerPlayed>()
            .init_state::<OpponentPlayed>()
            .add_event::<DrawEvent>()
            .add_systems(PostStartup, enable_anima::<With<CardLocation>>)
            .add_systems(PostStartup, set_turn_phase(&TurnPhase::Starting))
            .add_systems(OnEnter(TurnPhase::Starting), on_turn_starting)
            .add_systems(
                Update,
                (
                    (
                        apply_hover_material_on::<Hover>(Some(CardLocation::Hand)),
                        play_player_card,
                    )
                        .run_if(in_state(PlayerPlayed::No)),
                    play_opponent_card.run_if(in_state(OpponentPlayed::No)),
                    check_for_playing_phase_done,
                )
                    .run_if(in_state(TurnPhase::Playing)),
            )
            .add_systems(
                Update,
                transition_on_timer(&TurnPhase::Resolving, &TurnPhase::Ending),
            )
            .add_systems(OnEnter(TurnPhase::Ending), on_turn_ending)
            .add_systems(
                Update,
                transition_on_timer(&TurnPhase::Ending, &TurnPhase::Starting),
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
                ),
            )
            // Components
            .register_type::<State<PlayerPlayed>>()
            .register_type::<NextState<PlayerPlayed>>()
            .register_type::<State<OpponentPlayed>>()
            .register_type::<NextState<OpponentPlayed>>();
    }
}
