use std::{collections::HashMap, hash::Hash};

pub struct TrackedHashMap<K, V> {
    inner: HashMap<K, V>,
    dirty: bool,
}

impl<K: Eq + Hash, V> TrackedHashMap<K, V> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
            dirty: false,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.dirty = true;
        self.inner.insert(key, value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let res = self.inner.remove(key);
        if res.is_some() {
            self.dirty = true;
        }
        res
    }

    /// Returns true if changed since last check, and resets the flag
    pub fn take_dirty(&mut self) -> bool {
        std::mem::take(&mut self.dirty)
    }

    /// Peek without resetting
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    // Delegate read-only methods directly
    pub fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.dirty = true;
        self.inner.get_mut(key)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, K, V> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<'_, K, V> {
        self.dirty = true;
        self.inner.iter_mut()
    }

    pub fn keys(&self) -> std::collections::hash_map::Keys<'_, K, V> {
        self.inner.keys()
    }

    pub fn values(&self) -> std::collections::hash_map::Values<'_, K, V> {
        self.inner.values()
    }

    pub fn values_mut(&mut self) -> std::collections::hash_map::ValuesMut<'_, K, V> {
        self.dirty = true;
        self.inner.values_mut()
    }

    pub fn clear(&mut self) {
        if !self.inner.is_empty() {
            self.dirty = true;
            self.inner.clear();
        }
    }
}

impl<K: Eq + Hash, V> Default for TrackedHashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

// Implement IntoIterator for convenient for-loops
impl<'a, K: Eq + Hash, V> IntoIterator for &'a TrackedHashMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = std::collections::hash_map::Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a, K: Eq + Hash, V> IntoIterator for &'a mut TrackedHashMap<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = std::collections::hash_map::IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}
