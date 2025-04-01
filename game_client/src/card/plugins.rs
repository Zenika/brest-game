use anima::enable_anima;
use bevy::prelude::*;

use super::{
    components::{CardLocation, CardType, DeckSeqStamp, GraveyardSeqStamp, HandSeqStamp},
    resources::{CardColors, CardMaterials, CardMesh, DeckSeq, GraveyardSeq, HandSeq},
    systems::{
        arrange_deck, arrange_graveyard, arrange_hand, place_on_board, setup_cards, update_seqs,
    },
};

pub struct CardPlugin {
    pub base_card_color: Color,
    pub over_card_color: Color,
}

impl Plugin for CardPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(CardColors {
            base: self.base_card_color,
            over: self.over_card_color,
        })
        .init_resource::<CardMaterials>()
        .init_resource::<CardMesh>()
        .init_resource::<DeckSeq>()
        .init_resource::<HandSeq>()
        .init_resource::<GraveyardSeq>()
        .add_systems(Startup, setup_cards)
        .add_systems(PostStartup, enable_anima::<With<CardType>>)
        .add_systems(
            Update,
            (
                update_seqs,
                place_on_board,
                arrange_deck,
                arrange_hand,
                arrange_graveyard,
            ),
        )
        .register_type::<CardType>()
        .register_type::<CardLocation>()
        .register_type::<CardColors>()
        .register_type::<CardMaterials>()
        .register_type::<CardMesh>()
        .register_type::<DeckSeq>()
        .register_type::<DeckSeqStamp>()
        .register_type::<HandSeq>()
        .register_type::<HandSeqStamp>()
        .register_type::<GraveyardSeq>()
        .register_type::<GraveyardSeqStamp>();
    }
}
