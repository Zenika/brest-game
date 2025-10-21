pub trait GetTRSMut<T, R, S> {
    fn get_translation_mut(&mut self) -> &mut T;
    fn get_rotation_mut(&mut self) -> &mut R;
    fn get_scale_mut(&mut self) -> &mut S;
}

pub trait SetTRS<T, R, S> {
    fn set_translation(&mut self, translation: impl Into<T>) -> &mut Self;
    fn set_rotation(&mut self, rotation: impl Into<R>) -> &mut Self;
    fn set_scale(&mut self, scale: impl Into<S>) -> &mut Self;
}

impl<T, R, S, U: GetTRSMut<T, R, S>> SetTRS<T, R, S> for U {
    fn set_translation(&mut self, translation: impl Into<T>) -> &mut Self {
        *self.get_translation_mut() = translation.into();

        self
    }

    fn set_rotation(&mut self, rotation: impl Into<R>) -> &mut Self {
        *self.get_rotation_mut() = rotation.into();

        self
    }

    fn set_scale(&mut self, scale: impl Into<S>) -> &mut Self {
        *self.get_scale_mut() = scale.into();

        self
    }
}
