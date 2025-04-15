use bevy::prelude::*;

use super::{CardEvent, CardLocation, LocatedCardEvent, locate_card_event};

pub struct CardLocationPlugin;

impl Plugin for CardLocationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CardEvent<Pointer<Out>>>()
            .add_event::<CardEvent<Pointer<Over>>>()
            .add_event::<CardEvent<Pointer<Down>>>()
            .add_event::<CardEvent<Pointer<Up>>>()
            .add_event::<CardEvent<Pointer<Click>>>()
            .add_event::<LocatedCardEvent<Pointer<Out>>>()
            .add_event::<LocatedCardEvent<Pointer<Over>>>()
            .add_event::<LocatedCardEvent<Pointer<Down>>>()
            .add_event::<LocatedCardEvent<Pointer<Up>>>()
            .add_event::<LocatedCardEvent<Pointer<Click>>>()
            .add_systems(
                Update,
                (
                    locate_card_event::<Pointer<Over>>,
                    locate_card_event::<Pointer<Out>>,
                    locate_card_event::<Pointer<Down>>,
                    locate_card_event::<Pointer<Up>>,
                    locate_card_event::<Pointer<Click>>,
                ),
            )
            .register_type::<CardLocation>();
    }
}
