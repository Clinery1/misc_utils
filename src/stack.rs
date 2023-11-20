use serde::{Serialize, Deserialize};
use std::ops::{
    Index,
    IndexMut,
};


/// A simple stack data structure. Implements [`Index`] and [`IndexMut`] to index from the top of
/// the stack downward.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stack<T>(Vec<T>);
impl<T> Stack<T> {
    /// Creates a new stack
    pub fn new()->Self {
        Stack(Vec::with_capacity(4))
    }

    /// Creates a new stack with at least `capacity` empty slots
    pub fn with_capacity(capacity: usize)->Self {
        Stack(Vec::with_capacity(capacity.max(4)))
    }

    /// Pushes the item onto the stack
    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }

    /// Pops the last item or panics if there are no items
    pub fn pop(&mut self)->T {
        self.0.pop().expect("Attempt to pop an empty stack")
    }

    /// Gets a reference to the last item or panics if there are none
    pub fn last(&self)->&T {
        self.0.last().expect("Empty stack")
    }

    /// Gets a mutable reference to the last item or panics if there are none
    pub fn last_mut(&mut self)->&mut T {
        self.0.last_mut().expect("Empty stack")
    }
}
impl<T> Index<usize> for Stack<T> {
    type Output = T;
    fn index(&self, index: usize)->&T {
        &self.0[self.0.len().saturating_sub(index)]
    }
}
impl<T> IndexMut<usize> for Stack<T> {
    fn index_mut(&mut self, index: usize)->&mut T {
        let index = self.0.len().saturating_sub(index);
        &mut self.0[index]
    }
}
