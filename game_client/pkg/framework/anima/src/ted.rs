use crate::ted_seal::SetTED;

pub trait GetTED<T, E, D> {
    fn get_transform(&self) -> &T;
    fn get_easings(&self) -> &E;
    fn get_durations(&self) -> &D;
}

pub trait WithTED<T, E, D> {
    fn with_transform(&self, transform: impl Into<T>) -> Self;
    fn with_easings(&self, easings: impl Into<E>) -> Self;
    fn with_durations(&self, durations: impl Into<D>) -> Self;
}

impl<T, E, D, U: SetTED<T, E, D> + Clone> WithTED<T, E, D> for U {
    fn with_transform(&self, transform: impl Into<T>) -> Self {
        let mut clone = self.clone();

        clone.set_transform(transform);

        clone
    }

    fn with_easings(&self, easings: impl Into<E>) -> Self {
        let mut clone = self.clone();

        clone.set_easings(easings);

        clone
    }

    fn with_durations(&self, durations: impl Into<D>) -> Self {
        let mut clone = self.clone();

        clone.set_durations(durations);

        clone
    }
}
