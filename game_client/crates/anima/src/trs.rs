pub trait GetTRS<T, R, S> {
    fn get_translation(&self) -> &T;
    fn get_rotation(&self) -> &R;
    fn get_scale(&self) -> &S;
}

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

pub trait WithTRS<T, R, S> {
    fn with_translation(&self, translation: impl Into<T>) -> Self;
    fn with_rotation(&self, rotation: impl Into<R>) -> Self;
    fn with_scale(&self, scale: impl Into<S>) -> Self;
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

impl<T, R, S, U: SetTRS<T, R, S> + Clone> WithTRS<T, R, S> for U {
    fn with_translation(&self, translation: impl Into<T>) -> Self {
        let mut clone = self.clone();

        clone.set_translation(translation);

        clone
    }

    fn with_rotation(&self, rotation: impl Into<R>) -> Self {
        let mut clone = self.clone();

        clone.set_rotation(rotation);

        clone
    }

    fn with_scale(&self, scale: impl Into<S>) -> Self {
        let mut clone = self.clone();

        clone.set_scale(scale);

        clone
    }
}
