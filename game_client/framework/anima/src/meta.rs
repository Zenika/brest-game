use bevy::{animation::AnimationTargetId, prelude::*};

#[derive(Reflect, Clone, PartialEq)]
pub struct Meta {
    pub target_id: AnimationTargetId,
    pub clip_handle: Handle<AnimationClip>,
    pub node_index: AnimationNodeIndex,
}
