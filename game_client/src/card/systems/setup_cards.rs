use bevy::ecs::system::Commands;

use crate::card::{commands::SpawnCardExt, fixtures::CARDS};

pub fn setup_cards(mut commands: Commands) {
    for (index, card_type) in CARDS.into_iter().enumerate() {
        commands.spawn_card(format!("Card {}", index), card_type);
    }
}
