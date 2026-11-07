use bevy::animation::AnimationTargetId;
use bevy::prelude::*;

#[derive(Reflect, Clone, PartialEq)]
pub struct Meta {
    pub target_id: AnimationTargetId,
    pub clip_handle: Handle<AnimationClip>,
    pub node_index: AnimationNodeIndex,
}
