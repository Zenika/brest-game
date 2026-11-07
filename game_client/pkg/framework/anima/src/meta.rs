use bevy::prelude::*;

#[derive(Reflect, Clone, PartialEq)]
pub struct Meta {
    pub clip_handle: Handle<AnimationClip>,
    pub node_index: AnimationNodeIndex,
}
