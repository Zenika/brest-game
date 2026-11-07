use bevy::prelude::*;
use bevy_picking::events::{
    PointerClick, PointerOver, PointerOut, PointerPress, PointerRelease,
};
use entity_message::EntityMessage;

use super::{Deck, Field, Graveyard, Hand, LocatedEntityMessage};
use crate::playground::locate_card_entity_message;

pub struct PlaygroundLocationPlugin;

impl PlaygroundLocationPlugin {
    fn handle_event<E: Event + Clone>(app: &mut App) {
        app.add_message::<EntityMessage<E>>()
            .add_message::<LocatedEntityMessage<E, Deck>>()
            .add_message::<LocatedEntityMessage<E, Hand>>()
            .add_message::<LocatedEntityMessage<E, Field>>()
            .add_message::<LocatedEntityMessage<E, Graveyard>>()
            .add_systems(
                Update,
                (
                    locate_card_entity_message::<E, Deck>,
                    locate_card_entity_message::<E, Hand>,
                    locate_card_entity_message::<E, Field>,
                    locate_card_entity_message::<E, Graveyard>,
                ),
            );
    }
}

impl Plugin for PlaygroundLocationPlugin {
    fn build(&self, app: &mut App) {
        PlaygroundLocationPlugin::handle_event::<PointerOut>(app);
        PlaygroundLocationPlugin::handle_event::<PointerOver>(app);
        PlaygroundLocationPlugin::handle_event::<PointerPress>(app);
        PlaygroundLocationPlugin::handle_event::<PointerRelease>(app);
        PlaygroundLocationPlugin::handle_event::<PointerClick>(app);
    }
}
