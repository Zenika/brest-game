use bevy::{animation::AnimationTargetId, prelude::*};

mod ted;
mod trs;

use super::{Durations, Easings, Meta, Transform};

#[derive(Component, Reflect, Clone, PartialEq)]
#[reflect(Component)]
#[require(Transform, AnimationGraphHandle, AnimationPlayer)]
pub struct Anima {
    transform: Transform,
    easings: Easings,
    durations: Durations,
    meta: Meta,
}

impl Anima {
    pub fn new(
        target_id: AnimationTargetId,
        clip_handle: Handle<AnimationClip>,
        node_index: AnimationNodeIndex,
    ) -> Self {
        Self {
            transform: Transform::default(),
            easings: Easings::default(),
            durations: Durations::default(),
            meta: Meta {
                target_id,
                clip_handle,
                node_index,
            },
        }
    }

    pub fn target_id(&self) -> AnimationTargetId {
        self.meta.target_id
    }

    pub fn clip_handle(&self) -> Handle<AnimationClip> {
        self.meta.clip_handle.clone()
    }

    pub fn node_index(&self) -> AnimationNodeIndex {
        self.meta.node_index
    }
}
