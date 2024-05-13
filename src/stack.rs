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

    /// Clears the stack
    pub fn clear(&mut self) {
        self.0.clear();
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
    pub fn pop(&mut self)->Option<T> {
        self.0.pop()
    }

    /// Gets a reference to the last item or panics if there are none
    pub fn last(&self)->Option<&T> {
        self.0.last()
    }

    /// Gets a mutable reference to the last item or panics if there are none
    pub fn last_mut(&mut self)->Option<&mut T> {
        self.0.last_mut()
    }

    /// Iterate from the top of the stack to the bottom.
    pub fn iter<'a>(&'a self)->impl 'a + Iterator<Item = &'a T> {
        self.0.iter().rev()
    }

    /// Iterate from the top of the stack to the bottom, mutably.
    pub fn iter_mut<'a>(&'a mut self)->impl 'a + Iterator<Item = &'a mut T> {
        self.0.iter_mut().rev()
    }

    pub fn len(&self)->usize {self.0.len()}
}
impl<T> Index<usize> for Stack<T> {
    type Output = T;
    fn index(&self, index: usize)->&T {
        let index = self.0.len().saturating_sub(index + 1);
        &self.0[index]
    }
}
impl<T> IndexMut<usize> for Stack<T> {
    fn index_mut(&mut self, index: usize)->&mut T {
        let index = self.0.len().saturating_sub(index + 1);
        &mut self.0[index]
    }
}
