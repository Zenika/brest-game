pub trait GetTED<T, E, D> {
    fn get_transform(&self) -> &T;
    fn get_easings(&self) -> &E;
    fn get_durations(&self) -> &D;
}

pub trait GetTEDMut<T, E, D> {
    fn get_transform_mut(&mut self) -> &mut T;
    fn get_easings_mut(&mut self) -> &mut E;
    fn get_durations_mut(&mut self) -> &mut D;
}

pub trait SetTED<T, E, D> {
    fn set_transform(&mut self, transform: impl Into<T>) -> &mut Self;
    fn set_easings(&mut self, easings: impl Into<E>) -> &mut Self;
    fn set_durations(&mut self, durations: impl Into<D>) -> &mut Self;
}

pub trait WithTED<T, E, D> {
    fn with_transform(&self, transform: impl Into<T>) -> Self;
    fn with_easings(&self, easings: impl Into<E>) -> Self;
    fn with_durations(&self, durations: impl Into<D>) -> Self;
}

// impl<T, E, D, U: GetTEDMut<T, E, D>> SetTED<T, E, D> for U {
//     fn set_transform(&mut self, transform: impl Into<T>) -> &mut Self {
//         *self.get_transform_mut() = transform.into();

//         self
//     }

//     fn set_easings(&mut self, easings: impl Into<E>) -> &mut Self {
//         *self.get_easings_mut() = easings.into();

//         self
//     }

//     fn set_durations(&mut self, durations: impl Into<D>) -> &mut Self {
//         *self.get_durations_mut() = durations.into();

//         self
//     }
// }

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
