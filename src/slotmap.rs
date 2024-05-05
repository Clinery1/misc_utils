use serde::{Serialize, Deserialize};
use std::ops::{
    Index,
    IndexMut,
};
use crate::Key;


#[derive(Serialize, Deserialize, Debug, Clone)]
enum Slot<T> {
    Value(T),
    Reserved,
    None,
}
impl<T> Slot<T> {
    pub fn take(&mut self)->Option<T> {
        match std::mem::replace(self, Self::None) {
            Self::Value(t)=>Some(t),
            _=>None,
        }
    }

    #[inline]
    pub fn insert(&mut self, data: T) {
        *self = Self::Value(data);
    }

    pub fn is_reserved(&self)->bool {
        match self {
            Self::Reserved=>true,
            _=>false,
        }
    }

    pub fn has_data(&self)->bool {
        match self {
            Self::Value(_)=>true,
            _=>false,
        }
    }

    pub fn as_ref(&self)->Option<&T> {
        match self {
            Self::Value(t)=>Some(t),
            _=>None,
        }
    }

    pub fn as_mut(&mut self)->Option<&mut T> {
        match self {
            Self::Value(t)=>Some(t),
            _=>None,
        }
    }
}


/// A simple map of key:value that reuses old keys that are removed. DOES NOT solve the ABA
/// problem. The user (me) assumes all responsibility to ensure all keys are used properly.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SlotMap<K: Key, T> {
    inner: Vec<Slot<T>>,
    free: Vec<K>,
}
impl<K: Key, T> SlotMap<K, T> {
    fn get_slot(&mut self)->K {
        let k;
        if let Some(key) = self.free.pop() {
            k = key;
        } else {
            k = K::from_id(self.inner.len());
            self.inner.push(Slot::Reserved);
        }

        return k;
    }

    /// if the key points to a reserved slot or some data we are storing
    fn is_key_valid(&self, k: &K)->bool {
        if k.id() < self.inner.len() {
            let d = &self.inner[k.id()];
            d.has_data() || d.is_reserved()
        } else {
            false
        }
    }

    pub fn new()->Self {
        SlotMap {
            inner: Vec::new(),
            free: Vec::new(),
        }
    }

    pub fn insert(&mut self, data: T)->K {
        let key = self.get_slot();
        self.inner[key.id()].insert(data);

        return key;
    }

    #[inline]
    pub fn reserve_slot(&mut self)->K {
        self.get_slot()
    }

    /// Returns Err(data) when the key DOES NOT point to a valid reserved entry.
    pub fn insert_reserved(&mut self, key: K, data: T)->Result<(), T> {
        if !self.is_key_valid(&key) {
            return Err(data);
        }
        if !self.inner[key.id()].is_reserved() {
            return Err(data);
        }

        self.inner[key.id()].insert(data);

        return Ok(());
    }

    pub fn get(&self, key: K)->Option<&T> {
        if !self.is_key_valid(&key) {return None}

        let id = key.id();
        return self.inner[id].as_ref();
    }

    pub fn get_mut(&mut self, key: K)->Option<&mut T> {
        if !self.is_key_valid(&key) {return None}

        let id = key.id();
        return self.inner[id].as_mut();
    }

    pub fn remove(&mut self, key: K)->Option<T> {
        if !self.is_key_valid(&key) {return None}

        let id = key.id();
        return self.inner[id].take();
    }

    pub fn key_of_last_item(&self)->Option<K> {
        if self.inner.is_empty() {
            None
        } else {
            Some(K::from_id(self.inner.len() - 1))
        }
    }
}
impl<K: Key, T> Index<K> for SlotMap<K, T> {
    type Output = T;
    #[inline]
    fn index(&self, key: K)->&T {
        self.get(key).unwrap()
    }
}
impl<K: Key, T> IndexMut<K> for SlotMap<K, T> {
    #[inline]
    fn index_mut(&mut self, key: K)->&mut T {
        self.get_mut(key).unwrap()
    }
}
