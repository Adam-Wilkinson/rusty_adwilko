use std::{fmt::Debug, ops::AddAssign};

extern crate num;

#[derive(Clone, Copy, Debug)]
pub struct WithError<T>
    where T : Copy + Debug {
    pub value : T,
    pub error : f64,
}

impl<T> AddAssign for WithError<T>
    where T : AddAssign + Copy + Debug {
        fn add_assign(&mut self, rhs: Self) {
            self.value += rhs.value;
            self.error = self.error.max(rhs.error);
        }
}