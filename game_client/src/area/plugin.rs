use bevy::prelude::*;

use super::{
    Area, CardEvent, Deck, Graveyard, Hand, LocatedCardEvent, Opponent, Played, Player,
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
        app.add_event::<CardEvent<E>>()
            .add_event::<LocatedCardEvent<E, Deck>>()
            .add_event::<LocatedCardEvent<E, Hand>>()
            .add_event::<LocatedCardEvent<E, Played>>()
            .add_event::<LocatedCardEvent<E, Graveyard>>()
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
        PlaygroundAreaPlugin::build_event::<Pointer<Pressed>>(app);
        PlaygroundAreaPlugin::build_event::<Pointer<Released>>(app);
        PlaygroundAreaPlugin::build_event::<Pointer<Click>>(app);
    }
}

pub fn locate_card_event<E: Event, Location: Area>(
    mut card_events: EventReader<CardEvent<E>>,
    mut located_events: EventWriter<LocatedCardEvent<E, Location>>,
) {
    for event in card_events.read() {
        located_events.write(LocatedCardEvent::new(event.entity()));
    }
}
