use bevy::prelude::*;

mod ted;
mod trs;

use super::{Durations, Easings, Meta};
use crate::transforms::Transforms;

#[derive(Component, Reflect, Clone, PartialEq)]
#[reflect(Component)]
#[require(Transform, AnimationGraphHandle, AnimationPlayer)]
pub struct Anima {
    transforms: Transforms,
    easings: Easings,
    durations: Durations,
    meta: Meta,
}

impl Anima {
    pub fn new(clip_handle: Handle<AnimationClip>, node_index: AnimationNodeIndex) -> Self {
        Self {
            transforms: Transforms::default(),
            easings: Easings::default(),
            durations: Durations::default(),
            meta: Meta { clip_handle, node_index },
        }
    }

    pub fn clip_handle(&self) -> Handle<AnimationClip> {
        self.meta.clip_handle.clone()
    }

    pub fn node_index(&self) -> AnimationNodeIndex {
        self.meta.node_index
    }
}
