use anima::enable_anima;
use bevy::prelude::*;

use super::{
    components::*,
    events::*,
    resources::{reveal_timer::PhaseTimer, *},
    states::{OpponentPlayed, PlayerPlayed, TurnState},
    systems::{
        on_turn_starting::{on_ending, on_turn_starting},
        play_your_card::{play_opponent_card, play_your_card, trigger_resolve},
        *,
    },
};

pub struct CardPlugin {
    pub base_card_color: Color,
    pub hover_card_color: Color,
}

impl Plugin for CardPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(BaseCardColor(self.base_card_color))
            .insert_resource(HoverCardColor(self.hover_card_color))
            .insert_resource(PhaseTimer(Timer::from_seconds(1.0, TimerMode::Once)))
            .init_resource::<BaseCardMaterial>()
            .init_resource::<HoverCardMaterial>()
            .init_resource::<CardMesh>()
            .init_resource::<DeckSequence>()
            .init_resource::<HandSequence>()
            .init_resource::<GraveyardSequence>()
            .init_state::<TurnState>()
            .init_state::<PlayerPlayed>()
            .init_state::<OpponentPlayed>()
            .add_event::<CardEvent<Pointer<Out>>>()
            .add_event::<CardEvent<Pointer<Over>>>()
            .add_event::<CardEvent<Pointer<Click>>>()
            .add_event::<LocatedCardEvent<Pointer<Out>>>()
            .add_event::<LocatedCardEvent<Pointer<Over>>>()
            .add_event::<LocatedCardEvent<Pointer<Click>>>()
            .add_event::<DrawEvent>()
            .add_systems(Startup, setup_cards)
            .add_systems(PostStartup, enable_anima::<With<CardType>>)
            .add_systems(
                PostStartup,
                |mut next_state: ResMut<NextState<TurnState>>| next_state.set(TurnState::Starting),
            )
            .add_systems(
                OnEnter(TurnState::Resolving),
                |mut timer: ResMut<PhaseTimer>| timer.0.reset(),
            )
            .add_systems(
                OnEnter(TurnState::Ending),
                |mut timer: ResMut<PhaseTimer>| timer.0.reset(),
            )
            .add_systems(
                Update,
                (|time: Res<Time>,
                  mut timer: ResMut<PhaseTimer>,
                  mut next_state: ResMut<NextState<TurnState>>| {
                    if timer.0.tick(time.delta()).just_finished() {
                        next_state.set(TurnState::Ending);
                    }
                })
                .run_if(in_state(TurnState::Resolving)),
            )
            .add_systems(
                Update,
                (|time: Res<Time>,
                  mut timer: ResMut<PhaseTimer>,
                  mut next_state: ResMut<NextState<TurnState>>| {
                    if timer.0.tick(time.delta()).just_finished() {
                        next_state.set(TurnState::Starting);
                    }
                })
                .run_if(in_state(TurnState::Ending)),
            )
            .add_systems(OnEnter(TurnState::Ending), on_ending)
            .add_systems(
                Update,
                (
                    (
                        locate_card_event::<Pointer<Over>>,
                        locate_card_event::<Pointer<Out>>,
                        locate_card_event::<Pointer<Click>>,
                    ),
                    (
                        apply_material_on::<HoverCardMaterial, Pointer<Over>>(Some(
                            CardLocation::Board,
                        )),
                        apply_material_on::<BaseCardMaterial, Pointer<Out>>(None),
                    ),
                    (
                        apply_material_on::<HoverCardMaterial, Pointer<Over>>(Some(
                            CardLocation::Hand,
                        ))
                        .run_if(in_state(TurnState::Playing).and(in_state(PlayerPlayed::No))),
                        play_your_card
                            .run_if(in_state(TurnState::Playing).and(in_state(PlayerPlayed::No))),
                        play_opponent_card
                            .run_if(in_state(TurnState::Playing).and(in_state(OpponentPlayed::No))),
                    ),
                    (
                        increment_sequences,
                        place_on_board,
                        arrange_deck,
                        arrange_hand,
                        arrange_graveyard,
                        trigger_resolve.run_if(in_state(TurnState::Playing)),
                    ),
                )
                    .chain(),
            )
            .add_systems(OnEnter(TurnState::Starting), on_turn_starting)
            .add_systems(Update, (update_location_on_draw))
            // Components
            .register_type::<CardType>()
            .register_type::<CardLocation>()
            .register_type::<CardMesh>()
            .register_type::<DeckSequenceStamp>()
            .register_type::<HandSequenceStamp>()
            .register_type::<GraveyardSequenceStamp>()
            .register_type::<State<TurnState>>()
            .register_type::<NextState<TurnState>>()
            .register_type::<State<PlayerPlayed>>()
            .register_type::<NextState<PlayerPlayed>>()
            .register_type::<State<OpponentPlayed>>()
            .register_type::<NextState<OpponentPlayed>>()
            // Resources
            .register_type::<BaseCardColor>()
            .register_type::<HoverCardColor>()
            .register_type::<BaseCardMaterial>()
            .register_type::<HoverCardMaterial>()
            .register_type::<DeckSequence>()
            .register_type::<HandSequence>()
            .register_type::<GraveyardSequence>()
            .register_type::<PhaseTimer>();
    }
}
