use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug)]
pub struct Array<T> {
    elements: [T; 5],
}

impl<T> Array<T> {
    pub fn new(elements: [T; 5]) -> Self {
        Self { elements }
    }
}

impl<T> Deref for Array<T> {
    type Target = [T; 5];

    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

impl<T> DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements
    }
}
