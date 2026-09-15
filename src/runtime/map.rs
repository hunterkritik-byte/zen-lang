//! HashMap implementation for Zen
//! Supports key-value collections with insertion, lookup, and iteration

use std::collections::HashMap as StdHashMap;
use std::fmt;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct ZenMap<K: Eq + Hash + Clone, V: Clone> {
    data: StdHashMap<K, V>,
}

impl<K: Eq + Hash + Clone, V: Clone> ZenMap<K, V> {
    /// Create a new empty map
    pub fn new() -> Self {
        ZenMap {
            data: StdHashMap::new(),
        }
    }

    /// Get value by key
    pub fn get(&self, key: &K) -> Option<V> {
        self.data.get(key).cloned()
    }

    /// Insert key-value pair
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.data.insert(key, value)
    }

    /// Remove key from map
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }

    /// Check if key exists
    pub fn has_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    /// Get all keys
    pub fn keys(&self) -> Vec<K> {
        self.data.keys().cloned().collect()
    }

    /// Get all values
    pub fn values(&self) -> Vec<V> {
        self.data.values().cloned().collect()
    }

    /// Get number of entries
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if map is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Iterate over key-value pairs
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.data.iter()
    }
}

impl<K: Eq + Hash + Clone + fmt::Display, V: Clone + fmt::Display> fmt::Display for ZenMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        for (i, (k, v)) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", k, v)?;
        }
        write!(f, " }}")
    }
}

impl<K: Eq + Hash + Clone, V: Clone> Default for ZenMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let map = ZenMap::<String, i32>::new();
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_map_insert_get() {
        let mut map = ZenMap::new();
        map.insert("key1".to_string(), 42);
        assert_eq!(map.get(&"key1".to_string()), Some(42));
    }

    #[test]
    fn test_map_remove() {
        let mut map = ZenMap::new();
        map.insert("key".to_string(), 100);
        assert_eq!(map.remove(&"key".to_string()), Some(100));
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_map_keys_values() {
        let mut map = ZenMap::new();
        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);
        assert_eq!(map.keys().len(), 2);
        assert_eq!(map.values().len(), 2);
    }
}
