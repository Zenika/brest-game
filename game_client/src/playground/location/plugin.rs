use bevy::prelude::*;
use entity_message::EntityMessage;

use crate::playground::locate_card_entity_message;

use super::{Deck, Field, Graveyard, Hand, LocatedEntityMessage};

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
        PlaygroundLocationPlugin::handle_event::<Pointer<Out>>(app);
        PlaygroundLocationPlugin::handle_event::<Pointer<Over>>(app);
        PlaygroundLocationPlugin::handle_event::<Pointer<Press>>(app);
        PlaygroundLocationPlugin::handle_event::<Pointer<Release>>(app);
        PlaygroundLocationPlugin::handle_event::<Pointer<Click>>(app);
    }
}
