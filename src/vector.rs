use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug)]
pub struct Vector<T> {
    elements: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(elements: Vec<T>) -> Self {
        Self { elements }
    }
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements
    }
}
