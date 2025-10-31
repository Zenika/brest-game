use bevy::animation::{AnimationTarget, AnimationTargetId};
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

        let target_id = AnimationTargetId::from(&Name::new("Anima"));
        let clip_handle = animations.add(AnimationClip::default());
        let node_index = graph.add_clip(clip_handle.clone(), 1., graph.root);

        let anima = Anima::new(target_id, clip_handle, node_index)
            .set_translation((transform.translation, Easing::CubicInOut, 0.5))
            .set_rotation((transform.rotation, Easing::CubicInOut, 0.5))
            .set_scale((transform.scale, Easing::CubicInOut, 0.5));

        commands.entity(entity).insert((
            AnimationGraphHandle(graphs.add(graph)),
            anima,
            AnimationTarget { id: target_id, player: entity },
        ));
    }
}
