pub trait GetTED<'s, T, E, D> {
    fn get_transforms(&'s self) -> T;
    fn get_easings(&'s self) -> E;
    fn get_durations(&'s self) -> D;
}

pub trait SetTED<T, E, D> {
    fn set_transforms(self, transforms: impl Into<T>) -> Self;
    fn set_easings(self, easings: impl Into<E>) -> Self;
    fn set_durations(self, durations: impl Into<D>) -> Self;
}

pub trait WithTED<T, E, D> {
    fn with_transforms(&self, transforms: impl Into<T>) -> Self;
    fn with_easings(&self, easings: impl Into<E>) -> Self;
    fn with_durations(&self, durations: impl Into<D>) -> Self;
}

impl<T, E, D, U: SetTED<T, E, D> + Clone> WithTED<T, E, D> for U {
    fn with_transforms(&self, transforms: impl Into<T>) -> Self {
        self.clone().set_transforms(transforms)
    }

    fn with_easings(&self, easings: impl Into<E>) -> Self {
        self.clone().set_easings(easings)
    }

    fn with_durations(&self, durations: impl Into<D>) -> Self {
        self.clone().set_durations(durations)
    }
}
