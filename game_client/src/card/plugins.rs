use anima::enable_anima;
use bevy::prelude::*;

use super::{components::*, events::*, resources::*, systems::*};

pub struct CardPlugin {
    pub base_card_color: Color,
    pub hover_card_color: Color,
}

impl Plugin for CardPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(BaseCardColor(self.base_card_color))
            .insert_resource(HoverCardColor(self.hover_card_color))
            .init_resource::<BaseCardMaterial>()
            .init_resource::<HoverCardMaterial>()
            .init_resource::<CardMesh>()
            .init_resource::<DeckSequence>()
            .init_resource::<HandSequence>()
            .init_resource::<GraveyardSequence>()
            .add_event::<CardEvent<Pointer<Out>>>()
            .add_event::<CardEvent<Pointer<Over>>>()
            .add_event::<CardEvent<Pointer<Click>>>()
            .add_event::<LocatedCardEvent<Pointer<Out>>>()
            .add_event::<LocatedCardEvent<Pointer<Over>>>()
            .add_event::<LocatedCardEvent<Pointer<Click>>>()
            .add_systems(Startup, setup_cards)
            .add_systems(PostStartup, enable_anima::<With<CardType>>)
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
                            CardLocation::Hand,
                        )),
                        apply_material_on::<HoverCardMaterial, Pointer<Over>>(Some(
                            CardLocation::Board,
                        )),
                        apply_material_on::<BaseCardMaterial, Pointer<Out>>(None),
                    ),
                    (
                        translocate_on::<Pointer<Click>>(CardLocation::Deck, CardLocation::Hand),
                        translocate_on::<Pointer<Click>>(CardLocation::Hand, CardLocation::Board),
                        translocate_on::<Pointer<Click>>(
                            CardLocation::Board,
                            CardLocation::Graveyard,
                        ),
                        translocate_on::<Pointer<Click>>(
                            CardLocation::Graveyard,
                            CardLocation::Deck,
                        ),
                    ),
                    (
                        increment_sequences,
                        place_on_board,
                        arrange_deck,
                        arrange_hand,
                        arrange_graveyard,
                    ),
                )
                    .chain(),
            )
            // Components
            .register_type::<CardType>()
            .register_type::<CardLocation>()
            .register_type::<CardMesh>()
            .register_type::<DeckSequenceStamp>()
            .register_type::<HandSequenceStamp>()
            .register_type::<GraveyardSequenceStamp>()
            // Resources
            .register_type::<BaseCardColor>()
            .register_type::<HoverCardColor>()
            .register_type::<BaseCardMaterial>()
            .register_type::<HoverCardMaterial>()
            .register_type::<DeckSequence>()
            .register_type::<HandSequence>()
            .register_type::<GraveyardSequence>();
    }
}
