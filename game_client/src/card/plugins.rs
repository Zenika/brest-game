use bevy::prelude::*;

use super::{
    components::{CardLocation, CardType},
    resources::{CardColors, CardMaterials, CardMesh},
    systems::{setup_cards, update_position},
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
        .add_systems(Startup, setup_cards)
        .add_systems(Update, update_position)
        .register_type::<CardType>()
        .register_type::<CardLocation>()
        .register_type::<CardColors>()
        .register_type::<CardMaterials>()
        .register_type::<CardMesh>();
    }
}
