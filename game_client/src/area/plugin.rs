use bevy::prelude::*;

use super::{
    Area, CardMessage, Deck, Graveyard, Hand, LocatedCardMessage, Opponent, Played, Player,
    PlaygroundPart,
};

pub struct ContestantPlugin;

impl Plugin for ContestantPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>().register_type::<Opponent>();
    }
}

pub struct AreaPlugin;

impl Plugin for AreaPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Deck>()
            .register_type::<Hand>()
            .register_type::<Played>()
            .register_type::<Graveyard>();
    }
}

pub struct PlaygroundAreaPlugin;

impl PlaygroundAreaPlugin {
    fn build_event<E: Event>(app: &mut App) {
        app.add_message::<CardMessage<E>>()
            .add_message::<LocatedCardMessage<E, Deck>>()
            .add_message::<LocatedCardMessage<E, Hand>>()
            .add_message::<LocatedCardMessage<E, Played>>()
            .add_message::<LocatedCardMessage<E, Graveyard>>()
            .add_systems(
                Update,
                (
                    locate_card_event::<E, Deck>,
                    locate_card_event::<E, Hand>,
                    locate_card_event::<E, Played>,
                    locate_card_event::<E, Graveyard>,
                ),
            );
    }
}

impl Plugin for PlaygroundAreaPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlaygroundPart>();

        PlaygroundAreaPlugin::build_event::<Pointer<Out>>(app);
        PlaygroundAreaPlugin::build_event::<Pointer<Over>>(app);
        PlaygroundAreaPlugin::build_event::<Pointer<Press>>(app);
        PlaygroundAreaPlugin::build_event::<Pointer<Release>>(app);
        PlaygroundAreaPlugin::build_event::<Pointer<Click>>(app);
    }
}

pub fn locate_card_event<E: Event, Location: Area>(
    mut card_messages: MessageReader<CardMessage<E>>,
    mut located_messages: MessageWriter<LocatedCardMessage<E, Location>>,
) {
    for message in card_messages.read() {
        located_messages.write(LocatedCardMessage::new(message.entity()));
    }
}
