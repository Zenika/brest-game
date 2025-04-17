#[forbid(missing_docs)]
#[allow(dead_code)]
pub trait GetTEDMut<T, E, D> {
    fn get_transform_mut(&mut self) -> &mut T;
    fn get_easings_mut(&mut self) -> &mut E;
    fn get_durations_mut(&mut self) -> &mut D;
}

#[forbid(missing_docs)]
pub trait SetTED<T, E, D> {
    fn set_transform(&mut self, transform: impl Into<T>) -> &mut Self;
    fn set_easings(&mut self, easings: impl Into<E>) -> &mut Self;
    fn set_durations(&mut self, durations: impl Into<D>) -> &mut Self;
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
