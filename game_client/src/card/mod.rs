use bevy::{
    app::{Plugin, Startup, Update},
    color::Color,
};
use card::{Card, CardLocation, CardType};
use systems::{setup_cards, update_card_position};

mod card;
mod constants;
mod systems;

pub struct CardPlugin {
    pub base_card_color: Color,
    pub hover_card_color: Color,
}

impl Plugin for CardPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            Startup,
            setup_cards(self.base_card_color, self.hover_card_color),
        )
        .add_systems(Update, update_card_position)
        .register_type::<Card>()
        .register_type::<CardLocation>()
        .register_type::<CardType>();
    }
}
