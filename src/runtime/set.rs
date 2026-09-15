//! HashSet implementation for Zen
//! Supports unique collections with membership testing and set operations

use std::collections::HashSet as StdHashSet;
use std::fmt;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct ZenSet<T: Eq + Hash + Clone> {
    data: StdHashSet<T>,
}

impl<T: Eq + Hash + Clone> ZenSet<T> {
    /// Create a new empty set
    pub fn new() -> Self {
        ZenSet {
            data: StdHashSet::new(),
        }
    }

    /// Add element to set
    pub fn add(&mut self, value: T) -> bool {
        self.data.insert(value)
    }

    /// Remove element from set
    pub fn remove(&mut self, value: &T) -> bool {
        self.data.remove(value)
    }

    /// Check if element is in set
    pub fn contains(&self, value: &T) -> bool {
        self.data.contains(value)
    }

    /// Get number of elements
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if set is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clear all elements
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Union of two sets
    pub fn union(&self, other: &ZenSet<T>) -> ZenSet<T> {
        let mut result = ZenSet::new();
        for item in self.data.iter() {
            result.add(item.clone());
        }
        for item in other.data.iter() {
            result.add(item.clone());
        }
        result
    }

    /// Intersection of two sets
    pub fn intersection(&self, other: &ZenSet<T>) -> ZenSet<T> {
        let mut result = ZenSet::new();
        for item in self.data.iter() {
            if other.contains(item) {
                result.add(item.clone());
            }
        }
        result
    }

    /// Difference of two sets
    pub fn difference(&self, other: &ZenSet<T>) -> ZenSet<T> {
        let mut result = ZenSet::new();
        for item in self.data.iter() {
            if !other.contains(item) {
                result.add(item.clone());
            }
        }
        result
    }

    /// Iterate over set
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
}

impl<T: Eq + Hash + Clone + fmt::Display> fmt::Display for ZenSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        for (i, item) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, " }}")
    }
}

impl<T: Eq + Hash + Clone> Default for ZenSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_creation() {
        let set = ZenSet::<i32>::new();
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_set_add_contains() {
        let mut set = ZenSet::new();
        assert!(set.add(1));
        assert!(!set.add(1)); // Duplicate
        assert!(set.contains(&1));
    }

    #[test]
    fn test_set_operations() {
        let mut s1 = ZenSet::new();
        s1.add(1);
        s1.add(2);
        s1.add(3);

        let mut s2 = ZenSet::new();
        s2.add(2);
        s2.add(3);
        s2.add(4);

        let union = s1.union(&s2);
        assert_eq!(union.len(), 4);

        let intersection = s1.intersection(&s2);
        assert_eq!(intersection.len(), 2);

        let difference = s1.difference(&s2);
        assert_eq!(difference.len(), 1);
    }
}
