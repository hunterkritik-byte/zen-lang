//! Option type implementation for Zen
//! Represents a nullable value without using null pointers

use std::fmt;

/// Option type representing Some(value) or None
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ZenOption<T> {
    Some(T),
    None,
}

impl<T: Clone> ZenOption<T> {
    /// Check if option is Some
    pub fn is_some(&self) -> bool {
        matches!(self, ZenOption::Some(_))
    }

    /// Check if option is None
    pub fn is_none(&self) -> bool {
        matches!(self, ZenOption::None)
    }

    /// Unwrap value or panic
    pub fn unwrap(self) -> T {
        match self {
            ZenOption::Some(v) => v,
            ZenOption::None => panic!("called unwrap on None value"),
        }
    }

    /// Unwrap value with custom message or panic
    pub fn expect(self, msg: &str) -> T {
        match self {
            ZenOption::Some(v) => v,
            ZenOption::None => panic!("{}", msg),
        }
    }

    /// Get reference to inner value
    pub fn as_ref(&self) -> ZenOption<&T> {
        match self {
            ZenOption::Some(v) => ZenOption::Some(v),
            ZenOption::None => ZenOption::None,
        }
    }

    /// Map over option value
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> ZenOption<U> {
        match self {
            ZenOption::Some(v) => ZenOption::Some(f(v)),
            ZenOption::None => ZenOption::None,
        }
    }

    /// Flat map over option value
    pub fn flat_map<U, F: FnOnce(T) -> ZenOption<U>>(self, f: F) -> ZenOption<U> {
        match self {
            ZenOption::Some(v) => f(v),
            ZenOption::None => ZenOption::None,
        }
    }

    /// Get value or default
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            ZenOption::Some(v) => v,
            ZenOption::None => default,
        }
    }
}

impl<T: fmt::Display> fmt::Display for ZenOption<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZenOption::Some(v) => write!(f, "Some({})", v),
            ZenOption::None => write!(f, "None"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_some() {
        let opt = ZenOption::Some(42);
        assert!(opt.is_some());
        assert!(!opt.is_none());
        assert_eq!(opt.unwrap(), 42);
    }

    #[test]
    fn test_option_none() {
        let opt: ZenOption<i32> = ZenOption::None;
        assert!(!opt.is_some());
        assert!(opt.is_none());
    }

    #[test]
    fn test_option_map() {
        let opt = ZenOption::Some(5);
        let result = opt.map(|x| x * 2);
        assert_eq!(result.unwrap(), 10);
    }

    #[test]
    fn test_option_unwrap_or() {
        let opt = ZenOption::None::<i32>;
        assert_eq!(opt.unwrap_or(42), 42);
    }
}
