use std::fmt::Debug;

use bevy::animation::animation_curves::{AnimatableCurve, AnimatableProperty, AnimationCurve};
use bevy::animation::{AnimationClip, AnimationPlayer, AnimationTargetId, animated_field};
use bevy::asset::Assets;
use bevy::curve::prelude::*;
use bevy::ecs::query::Changed;
use bevy::ecs::system::{Query, ResMut};
use bevy::reflect::{FromReflect, Reflectable};
use bevy::transform::components::Transform;

use super::{Anima, GetTRS};

pub fn animate_anima(
    mut animations: ResMut<Assets<AnimationClip>>,
    mut query: Query<(&Transform, &Anima, &AnimationTargetId, &mut AnimationPlayer), Changed<Anima>>,
) {
    for (transform, anima, target_id, mut player) in &mut query {
        player.stop(anima.node_index());

        let mut clip = animations.get_mut(&anima.clip_handle()).unwrap();

        clip.add_curve_to_target(*target_id, translation_curve(anima, transform));
        clip.add_curve_to_target(*target_id, rotation_curve(anima, transform));
        clip.add_curve_to_target(*target_id, scale_curve(anima, transform));

        player.play(anima.node_index());
    }
}

fn translation_curve(anima: &Anima, transform: &Transform) -> impl AnimationCurve {
    let current = transform.translation;
    let (&next, &easing, &duration) = anima.get_translation();

    property_curve(animated_field!(Transform::translation), current, next, easing, duration)
}

fn rotation_curve(anima: &Anima, transform: &Transform) -> impl AnimationCurve {
    let current = transform.rotation;
    let (&next, &easing, &duration) = anima.get_rotation();

    property_curve(animated_field!(Transform::rotation), current, next, easing, duration)
}

fn scale_curve(anima: &Anima, transform: &Transform) -> impl AnimationCurve {
    let current = transform.scale;
    let (&next, &easing, &duration) = anima.get_scale();

    property_curve(animated_field!(Transform::scale), current, next, easing, duration)
}

fn property_curve<P: AnimatableProperty + Clone>(
    property: P,
    current: P::Property,
    next: P::Property,
    easing: EaseFunction,
    duration: f32,
) -> impl AnimationCurve
where
    P::Property: Ease + Clone + FromReflect + Reflectable + Debug,
{
    let domain = interval(0., duration).unwrap();
    let curve = EasingCurve::new(current, next, easing).reparametrize_linear(domain).unwrap();
    AnimatableCurve::new(property, curve)
}
