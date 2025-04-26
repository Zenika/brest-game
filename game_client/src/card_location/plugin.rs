use bevy::prelude::*;

use super::{CardEvent, Deck, Graveyard, Hand, LocatedCardEvent, Played, locate_card_event};

pub struct CardLocationPlugin;

impl CardLocationPlugin {
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

impl Plugin for CardLocationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Deck>()
            .register_type::<Hand>()
            .register_type::<Played>()
            .register_type::<Graveyard>();

        CardLocationPlugin::build_event::<Pointer<Out>>(app);
        CardLocationPlugin::build_event::<Pointer<Over>>(app);
        CardLocationPlugin::build_event::<Pointer<Down>>(app);
        CardLocationPlugin::build_event::<Pointer<Up>>(app);
        CardLocationPlugin::build_event::<Pointer<Click>>(app);
    }
}
