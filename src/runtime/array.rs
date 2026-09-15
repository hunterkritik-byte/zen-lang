//! Array implementation for Zen
//! Supports dynamic growable arrays with bounds checking and iteration

use std::fmt;

#[derive(Clone, Debug)]
pub struct ZenArray<T> {
    data: Vec<T>,
}

impl<T: Clone> ZenArray<T> {
    /// Create a new empty array
    pub fn new() -> Self {
        ZenArray { data: Vec::new() }
    }

    /// Create array from vector
    pub fn from_vec(data: Vec<T>) -> Self {
        ZenArray { data }
    }

    /// Get the length of the array
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if array is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Access element by index with bounds checking
    pub fn get(&self, index: i32) -> Result<T, String> {
        if index < 0 || index >= self.data.len() as i32 {
            return Err(format!("Index {} out of bounds for array of length {}", index, self.data.len()));
        }
        Ok(self.data[index as usize].clone())
    }

    /// Set element at index
    pub fn set(&mut self, index: i32, value: T) -> Result<(), String> {
        if index < 0 || index >= self.data.len() as i32 {
            return Err(format!("Index {} out of bounds for array of length {}", index, self.data.len()));
        }
        self.data[index as usize] = value;
        Ok(())
    }

    /// Push element to end
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    /// Pop element from end
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Get slice of array
    pub fn slice(&self, start: i32, end: i32) -> Result<ZenArray<T>, String> {
        let len = self.data.len() as i32;
        let start = if start < 0 { 0 } else { start };
        let end = if end > len { len } else { end };

        if start >= end || start < 0 || end < 0 {
            return Err("Invalid slice range".to_string());
        }

        let slice = self.data[start as usize..end as usize].to_vec();
        Ok(ZenArray::from_vec(slice))
    }

    /// Reverse array
    pub fn reverse(&mut self) {
        self.data.reverse();
    }

    /// Iterate over array
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
}

impl<T: Clone + fmt::Display> fmt::Display for ZenArray<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let arr = ZenArray::<i32>::new();
        assert_eq!(arr.len(), 0);
    }

    #[test]
    fn test_array_push_pop() {
        let mut arr = ZenArray::new();
        arr.push(1);
        arr.push(2);
        arr.push(3);
        assert_eq!(arr.len(), 3);
        assert_eq!(arr.pop(), Some(3));
        assert_eq!(arr.len(), 2);
    }

    #[test]
    fn test_array_indexing() {
        let mut arr = ZenArray::new();
        arr.push(10);
        arr.push(20);
        assert_eq!(arr.get(0).unwrap(), 10);
        assert_eq!(arr.get(1).unwrap(), 20);
    }

    #[test]
    fn test_array_bounds_checking() {
        let arr = ZenArray::<i32>::from_vec(vec![1, 2, 3]);
        assert!(arr.get(5).is_err());
        assert!(arr.get(-1).is_err());
    }

    #[test]
    fn test_array_slice() {
        let arr = ZenArray::from_vec(vec![1, 2, 3, 4, 5]);
        let slice = arr.slice(1, 4).unwrap();
        assert_eq!(slice.len(), 3);
    }
}
