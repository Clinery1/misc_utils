use serde::{Serialize, Deserialize};
use std::ops::{
    Index,
    IndexMut,
};
use crate::Key;


/// A simple map of key:value that reuses old keys that are removed. DOES NOT solve the ABA
/// problem. The user (me) assumes all responsibility to ensure all keys are used properly.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SlotMap<K: Key, T> {
    inner: Vec<Option<T>>,
    free: Vec<K>,
}
impl<K: Key, T> SlotMap<K, T> {
    pub fn new()->Self {
        SlotMap {
            inner: Vec::new(),
            free: Vec::new(),
        }
    }

    pub fn insert(&mut self, data: T)->K {
        let key = self.free.pop().unwrap_or(K::from_id(self.inner.len()));
        self.inner[key.id()] = Some(data);

        return key;
    }

    /// assumes the key is valid
    pub fn get(&self, key: K)->&T {
        let id = key.id();
        assert!(id < self.inner.len());
        assert!(self.inner[id].is_some());

        return self.inner[id].as_ref().unwrap();
    }

    /// assumes the key is valid
    pub fn get_mut(&mut self, key: K)->&mut T {
        let id = key.id();
        assert!(id < self.inner.len());
        assert!(self.inner[id].is_some());

        return self.inner[id].as_mut().unwrap();
    }

    /// assumes the key is valid
    pub fn remove(&mut self, key: K)->T {
        let id = key.id();
        assert!(id < self.inner.len());
        assert!(self.inner[id].is_some());

        return self.inner[id].take().unwrap();
    }
}
impl<K: Key, T> Index<K> for SlotMap<K, T> {
    type Output = T;
    #[inline]
    fn index(&self, key: K)->&T {
        self.get(key)
    }
}
impl<K: Key, T> IndexMut<K> for SlotMap<K, T> {
    #[inline]
    fn index_mut(&mut self, key: K)->&mut T {
        self.get_mut(key)
    }
}
