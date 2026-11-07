use anima::enable_anima;
use bevy::prelude::*;
use shared::{ContestantID, Play};
use states_timer::set_on_timer;

use super::messages::*;
use super::resources::{OpponentID, PlayerID};
use super::systems::*;
use crate::card::{BaseCardMaterial, HoverCardMaterial, apply_material_on};
use crate::flow::{BattlePhase, ContestantPlayed, Opponent, OpponentPlayed, Player, PlayerPlayed, RoundPhase};
use crate::playground::{Deck, Field, Graveyard, Hand};

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
            .add_message::<DrawMessage>()
            .add_systems(PostStartup, enable_anima::<With<Mesh3d>>)
            .add_systems(Update, set_on_timer(BattlePhase::Started, BattlePhase::InProgress))
            .add_systems(OnEnter(RoundPhase::Starting), on_round_starting)
            .add_systems(
                Update,
                (
                    apply_material_on::<HoverCardMaterial, Pointer<Over>, Hand>.in_set(PlayerPlayedSet::No),
                    request_player_play.in_set(PlayerPlayedSet::No),
                    request_opponent_play.in_set(OpponentPlayedSet::No),
                    end_playing_phase_policy,
                )
                    .in_set(RoundPhase::Playing),
            )
            .add_systems(Update, set_on_timer(RoundPhase::Resolving, RoundPhase::Ending))
            .add_systems(OnEnter(RoundPhase::Ending), on_round_ending)
            .add_systems(
                Update,
                (
                    apply_material_on::<BaseCardMaterial, Pointer<Out>, Deck>,
                    apply_material_on::<BaseCardMaterial, Pointer<Out>, Hand>,
                    apply_material_on::<BaseCardMaterial, Pointer<Out>, Field>,
                    apply_material_on::<BaseCardMaterial, Pointer<Out>, Graveyard>,
                    arrange_pile::<Player, Deck>,
                    arrange_fan::<Player, Hand>,
                    arrange_pile::<Player, Field>,
                    arrange_pile::<Player, Graveyard>,
                    arrange_pile::<Opponent, Deck>,
                    arrange_fan::<Opponent, Hand>,
                    arrange_pile::<Opponent, Field>,
                    arrange_pile::<Opponent, Graveyard>,
                    handle_draw_message,
                    handle_play.run_if(on_message::<Play>),
                    make_exclusive::<Deck, Hand>,
                    make_exclusive::<Deck, Field>,
                    make_exclusive::<Deck, Graveyard>,
                    make_exclusive::<Hand, Field>,
                    make_exclusive::<Hand, Graveyard>,
                    make_exclusive::<Field, Graveyard>,
                ),
            );
    }
}
