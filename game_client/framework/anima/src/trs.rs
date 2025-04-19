use crate::trs_seal::SetTRS;

pub trait GetTRS<T, R, S> {
    fn get_translation(&self) -> &T;
    fn get_rotation(&self) -> &R;
    fn get_scale(&self) -> &S;
}

pub trait WithTRS<T, R, S> {
    fn with_translation(&self, translation: impl Into<T>) -> Self;
    fn with_rotation(&self, rotation: impl Into<R>) -> Self;
    fn with_scale(&self, scale: impl Into<S>) -> Self;
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
