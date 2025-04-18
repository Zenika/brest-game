use bevy::prelude::*;
use entity_event::send_entity_event_on;

use crate::{
    card_location::{CardEvent, CardLocation},
    card_material::{BaseCardMaterial, CardMaterial},
    card_mesh::CardMesh,
    sequences::{DeckSequence, DeckSequenceStamp},
};

pub struct SpawnCard {
    pub name: String,
}

impl Command for SpawnCard {
    fn apply(self, world: &mut World) {
        let card_mesh = world.resource::<CardMesh>();
        let card_material = world.resource::<BaseCardMaterial>();

        let base_bundle = (Name::new(self.name), CardLocation::Deck);

        let rendering_bundle = (
            Transform::from_xyz(0., 0., 1.),
            Mesh3d((*card_mesh).clone()),
            MeshMaterial3d(card_material.as_material()),
        );

        let mut deck_seq: Mut<DeckSequence> = world.resource_mut::<DeckSequence>();
        let deck_seq_stamp = DeckSequenceStamp(deck_seq.next());

        let mut commands = world.commands();

        commands
            .spawn(base_bundle)
            .insert(rendering_bundle)
            .insert(deck_seq_stamp)
            .observe(send_entity_event_on::<Pointer<Over>, CardEvent<Pointer<Over>>>)
            .observe(send_entity_event_on::<Pointer<Out>, CardEvent<Pointer<Out>>>)
            .observe(send_entity_event_on::<Pointer<Click>, CardEvent<Pointer<Click>>>);
    }
}

pub trait SpawnCardExt {
    fn spawn_card(&mut self, name: String);
}

impl SpawnCardExt for Commands<'_, '_> {
    fn spawn_card(&mut self, name: String) {
        self.queue(SpawnCard { name });
    }
}
