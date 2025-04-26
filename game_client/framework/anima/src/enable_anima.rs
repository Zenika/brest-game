use bevy::{
    animation::{AnimationTarget, AnimationTargetId},
    ecs::query::QueryFilter,
    prelude::*,
};

use super::{Anima, Easing, WithTED};

pub fn enable_anima<F: QueryFilter>(
    mut commands: Commands,
    mut animations: ResMut<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    query: Query<(Entity, &Transform), F>,
) {
    for (entity, &transform) in &query {
        let mut graph = AnimationGraph::new();

        let target_id = AnimationTargetId::from(&Name::new("Anima"));
        let clip_handle = animations.add(AnimationClip::default());
        let node_index = graph.add_clip(clip_handle.clone(), 1., graph.root);

        // Anima Immutable (With) TED API (TED = Transform / Easings / Durations)
        let anima = Anima::new(target_id, clip_handle, node_index)
            .with_transform(transform)
            .with_easings((Easing::CubicInOut, Easing::CubicInOut, Easing::ElasticInOut))
            .with_durations((0.5, 0.5, 0.5));

        commands.entity(entity).insert((
            AnimationGraphHandle(graphs.add(graph)),
            anima,
            AnimationTarget {
                id: target_id,
                player: entity,
            },
        ));
    }
}
