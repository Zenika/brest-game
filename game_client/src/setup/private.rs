use bevy::ecs::system::Commands;

use crate::constants::CARDS_COUNT;

use super::SpawnCardExt;

pub fn setup_cards(mut commands: Commands) {
    for index in 0..CARDS_COUNT {
        commands.spawn_card(format!("Card {}", index));
    }
}
