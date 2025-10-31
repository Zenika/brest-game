pub trait GetTRS<'s, T, R, S> {
    fn get_translation(&'s self) -> T;
    fn get_rotation(&'s self) -> R;
    fn get_scale(&'s self) -> S;
}

pub trait SetTRS<T, R, S> {
    fn set_translation(self, translation: impl Into<T>) -> Self;
    fn set_rotation(self, rotation: impl Into<R>) -> Self;
    fn set_scale(self, scale: impl Into<S>) -> Self;
}

pub trait WithTRS<T, R, S> {
    fn with_translation(&self, translation: impl Into<T>) -> Self;
    fn with_rotation(&self, rotation: impl Into<R>) -> Self;
    fn with_scale(&self, scale: impl Into<S>) -> Self;
}

impl<T, R, S, U: SetTRS<T, R, S> + Clone> WithTRS<T, R, S> for U {
    fn with_translation(&self, translation: impl Into<T>) -> Self {
        self.clone().set_translation(translation)
    }

    fn with_rotation(&self, rotation: impl Into<R>) -> Self {
        self.clone().set_rotation(rotation)
    }

    fn with_scale(&self, scale: impl Into<S>) -> Self {
        self.clone().set_scale(scale)
    }
}
