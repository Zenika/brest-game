use bevy::prelude::*;

pub fn make_exclusive<C1: Component, C2: Component>(
    mut commands: Commands,
    query1: Query<Entity, (With<C1>, Added<C2>)>,
    query2: Query<Entity, (With<C2>, Added<C1>)>,
) {
    for entity in query1 {
        commands.entity(entity).remove::<C1>();
    }

    for entity in query2 {
        commands.entity(entity).remove::<C2>();
    }
}
