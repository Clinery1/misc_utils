//! Copied from my `ssa_optimization_parser` project where I try out a parser that generates an
//! SSA-like high-level IR and performs optimizations on it. That seems much easier to use than a
//! recursive AST. It is adapted to use the [`Key`] trait I have made.


use std::{
    ops::{
        Index,
        IndexMut,
    },
    slice::{
        Iter,
        IterMut,
    },
    iter::{
        Enumerate,
        IntoIterator,
        Extend,
    },
    marker::PhantomData,
};
use crate::Key;


/// A list that does not reuse or remove indices upon deletion. The exception is the `pop`
/// function. This function removes the last item from the backing list since it does not affect
/// the indices of the other items. Does not allow insertion of items to avoid messing up the order
/// of the other items. If you don't need removal, [`crate::keyed_vec::KeyedVec`] works the
/// similarly.
#[derive(Debug)]
pub struct SparseList<K: Key, T> {
    inner: Vec<Option<T>>,
    used_count: usize,
    _phantom: PhantomData<K>,
}
#[allow(dead_code)]
impl<K: Key, T> SparseList<K, T> {
    pub fn new()->Self {
        SparseList {
            inner: Vec::new(),
            used_count: 0,
            _phantom: PhantomData,
        }
    }
    
    pub fn push(&mut self, data: T)->K {
        self.inner.push(Some(data));
        self.used_count += 1;
        return K::from_id(self.inner.len() - 1);
    }

    pub fn remove(&mut self, index: usize)->Option<T> {
        self.used_count -= 1;
        self.inner[index].take()
    }

    pub fn pop(&mut self)->Option<T> {
        self.used_count -= 1;
        self.inner.pop().flatten()
    }

    pub fn iter<'a>(&'a self)->SparseListIter<'a, T> {
        SparseListIter(self.inner.iter())
    }

    pub fn used_count(&self)->usize {
        self.used_count
    }

    pub fn slot_count(&self)->usize {
        self.inner.len()
    }

    pub fn iter_mut<'a>(&'a mut self)->SparseListIterMut<'a, T> {
        SparseListIterMut(self.inner.iter_mut())
    }

    pub fn iter_keys<'a>(&'a self)->SparseListIterKeys<'a, K, T> {
        SparseListIterKeys {
            inner:self.inner.iter().enumerate(),
            _phantom: PhantomData,
        }
    }

    pub fn iter_mut_with_keys<'a>(&'a mut self)->SparseListIterMutKeys<'a, K, T> {
        SparseListIterMutKeys {
            inner: self.inner.iter_mut().enumerate(),
            _phantom: PhantomData,
        }
    }
}
impl<K: Key, T> Index<K> for SparseList<K, T> {
    type Output = Option<T>;
    fn index(&self, index: K)->&Self::Output {
        &self.inner[index.get_id()]
    }
}
impl<K: Key, T> IndexMut<K> for SparseList<K, T> {
    fn index_mut(&mut self, index: K)->&mut Self::Output {
        &mut self.inner[index.get_id()]
    }
}
impl<K: Key, T> IntoIterator for SparseList<K, T> {
    type Item = T;
    type IntoIter = SparseListIntoIter<T>;
    fn into_iter(self)->Self::IntoIter {
        SparseListIntoIter {
            inner: self.inner.into_iter(),
        }
    }
}
impl<K: Key, T> Extend<T> for SparseList<K, T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let starting_len = self.inner.len();
        self.inner.extend(iter.into_iter().map(Option::Some));
        let added_count = self.inner.len() - starting_len;

        self.used_count += added_count;
    }
}

pub struct SparseListIter<'a, T: 'a>(Iter<'a, Option<T>>);
impl<'a, T> Iterator for SparseListIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self)->Option<&'a T> {
        while let Some(i) = self.0.next() {
            if i.is_some() {
                return i.as_ref();
            }
        }

        return None;
    }
}

pub struct SparseListIterMut<'a, T: 'a>(IterMut<'a, Option<T>>);
impl<'a, T: 'a> Iterator for SparseListIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self)->Option<&'a mut T> {
        while let Some(i) = self.0.next() {
            if i.is_some() {
                return i.as_mut();
            }
        }

        return None;
    }
}

pub struct SparseListIterKeys<'a, K: Key, T: 'a> {
    inner: Enumerate<Iter<'a, Option<T>>>,
    _phantom: PhantomData<K>,
}
impl<'a, K: Key, T: 'a> Iterator for SparseListIterKeys<'a, K, T> {
    type Item = K;
    fn next(&mut self)->Option<K> {
        while let Some((i, t)) = self.inner.next() {
            if t.is_some() {
                return Some(K::from_id(i));
            }
        }

        return None;
    }
}

pub struct SparseListIterMutKeys<'a, K: Key, T: 'a> {
    inner: Enumerate<IterMut<'a, Option<T>>>,
    _phantom: PhantomData<K>,
}
impl<'a, K: Key, T: 'a> Iterator for SparseListIterMutKeys<'a, K, T> {
    type Item = (K, &'a mut T);
    fn next(&mut self)->Option<Self::Item> {
        while let Some((i, o_t)) = self.inner.next() {
            if let Some(t) = o_t {
                return Some((K::from_id(i), t));
            }
        }

        return None;
    }
}

pub struct SparseListIntoIter<T> {
    inner: std::vec::IntoIter<Option<T>>,
}
impl<T> Iterator for SparseListIntoIter<T> {
    type Item = T;
    fn next(&mut self)->Option<T> {
        while let Some(item) = self.inner.next() {
            if item.is_some() {
                return item;
            }
        }

        return None;
    }
}
