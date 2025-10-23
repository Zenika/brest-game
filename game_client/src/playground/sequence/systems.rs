use bevy::prelude::*;

use super::{Sequence, SequenceStamp};

pub fn increment_sequence<T: Component>(
    mut commands: Commands,
    mut sequence: ResMut<Sequence<T>>,
    query: Query<Entity, Added<T>>,
) {
    for entity in &query {
        commands
            .entity(entity)
            .insert(SequenceStamp::<T>::from(sequence.next()));
    }
}
