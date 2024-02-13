use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct Container<T> {
    elements: (T, T, T),
}

impl<T> Container<T> {
    pub fn new(elements: (T, T, T)) -> Self {
        Self { elements }
    }
}

impl<T> Deref for Container<T> {
    type Target = (T, T, T);

    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

impl<T> DerefMut for Container<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements
    }
}
