use bevy::prelude::*;
use entity_event::send_entity_event_on;

use crate::card::{
    components::{CardLocation, CardType, DeckSequenceStamp},
    events::CardEvent,
    resources::{BaseCardMaterial, CardMaterial, CardMesh, DeckSequence},
};

pub struct SpawnCard {
    pub name: String,
    pub card_type: CardType,
}

impl Command for SpawnCard {
    fn apply(self, world: &mut World) {
        let card_mesh = world.resource::<CardMesh>();
        let card_material = world.resource::<BaseCardMaterial>();

        let base_bundle = (Name::new(self.name), self.card_type, CardLocation::Deck);

        let rendering_bundle = (
            Transform::from_xyz(0., 0., 1.),
            Mesh3d((*card_mesh).clone()),
            MeshMaterial3d(card_material.as_material()),
        );

        let mut deck_seq: Mut<'_, DeckSequence> = world.resource_mut::<DeckSequence>();
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
    fn spawn_card(&mut self, name: String, card_type: CardType);
}

impl SpawnCardExt for Commands<'_, '_> {
    fn spawn_card(&mut self, name: String, card_type: CardType) {
        self.queue(SpawnCard { name, card_type });
    }
}
