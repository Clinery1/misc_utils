use serde::{Serialize, Deserialize};
use std::{
    ops::{
        Index,
        IndexMut,
    },
    marker::PhantomData,
};
use crate::Key;


/// A simple keyed list of data. Removal is not possible. Basically a `Vec<T>`, but avoids the
/// hassle of using raw `usize` to index a `Vec` and makes it a type error if the key comes from
/// another kind of map.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyedVec<K: Key, T> {
    inner: Vec<T>,
    _phantom: PhantomData<K>,
}
impl<K: Key, T> KeyedVec<K, T> {
    pub fn new()->Self {
        KeyedVec {
            inner: Vec::new(),
            _phantom: PhantomData,
        }
    }

    pub fn insert(&mut self, data: T)->K {
        let key = K::from_id(self.inner.len());
        self.inner.push(data);
        return key;
    }

    pub fn get(&self, key: K)->&T {
        let id = key.id();
        assert!(id < self.inner.len());
        return &self.inner[id];
    }

    pub fn get_mut(&mut self, key: K)->&mut T {
        let id = key.id();
        assert!(id < self.inner.len());
        return &mut self.inner[id];
    }
}
impl<K: Key, T> Index<K> for KeyedVec<K, T> {
    type Output = T;
    #[inline]
    fn index(&self, key: K)->&T {
        self.get(key)
    }
}
impl<K: Key, T> IndexMut<K> for KeyedVec<K, T> {
    #[inline]
    fn index_mut(&mut self, key: K)->&mut T {
        self.get_mut(key)
    }
}
