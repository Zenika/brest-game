use bevy::animation::{AnimatedBy, AnimationTargetId};
use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;

use super::{Anima, Easing};
use crate::SetTRS;

pub fn enable_anima<F: QueryFilter>(
    mut commands: Commands,
    mut animations: ResMut<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    query: Query<(Entity, &Transform), F>,
) {
    for (entity, &transform) in &query {
        let mut graph = AnimationGraph::new();

        let clip_handle = animations.add(AnimationClip::default());
        let node_index = graph.add_clip(clip_handle.clone(), 1., graph.root);

        let anima = Anima::new(clip_handle, node_index)
            .set_translation((transform.translation, Easing::CubicInOut, 0.5))
            .set_rotation((transform.rotation, Easing::CubicInOut, 0.5))
            .set_scale((transform.scale, Easing::CubicInOut, 0.5));

        commands.entity(entity).insert((
            anima,
            AnimatedBy(entity),
            AnimationGraphHandle(graphs.add(graph)),
            AnimationTargetId::from(&Name::new("Anima")),
        ));
    }
}
