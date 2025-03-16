use bevy::{animation::animated_field, prelude::*};

use super::{Anima, GetTED, GetTRS};

pub fn animate_anima(
    mut animations: ResMut<Assets<AnimationClip>>,
    mut query: Query<(&Transform, &Anima, &mut AnimationPlayer), Changed<Anima>>,
) {
    for (transform, anima, mut player) in &mut query {
        player.stop(anima.node_index());

        let clip = animations.get_mut(&anima.clip_handle()).unwrap();

        clip.add_curve_to_target(
            anima.target_id(),
            AnimatableCurve::new(
                animated_field!(Transform::translation),
                EasingCurve::new(
                    *transform.get_translation(),
                    *anima.get_transform().get_translation(),
                    *anima.get_easings().get_translation(),
                )
                .reparametrize_linear(
                    interval(0., *anima.get_durations().get_translation()).unwrap(),
                )
                .unwrap(),
            ),
        );

        clip.add_curve_to_target(
            anima.target_id(),
            AnimatableCurve::new(
                animated_field!(Transform::rotation),
                EasingCurve::new(
                    *transform.get_rotation(),
                    *anima.get_transform().get_rotation(),
                    *anima.get_easings().get_rotation(),
                )
                .reparametrize_linear(interval(0., *anima.get_durations().get_rotation()).unwrap())
                .unwrap(),
            ),
        );

        clip.add_curve_to_target(
            anima.target_id(),
            AnimatableCurve::new(
                animated_field!(Transform::scale),
                EasingCurve::new(
                    *transform.get_scale(),
                    *anima.get_transform().get_scale(),
                    *anima.get_easings().get_scale(),
                )
                .reparametrize_linear(interval(0., *anima.get_durations().get_scale()).unwrap())
                .unwrap(),
            ),
        );

        player.play(anima.node_index());
    }
}
