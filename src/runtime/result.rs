//! Result type implementation for Zen
//! Represents success (Ok) or failure (Err) without exceptions

use std::fmt;

/// Result type representing Ok(value) or Err(error)
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ZenResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T: Clone, E: Clone> ZenResult<T, E> {
    /// Check if result is Ok
    pub fn is_ok(&self) -> bool {
        matches!(self, ZenResult::Ok(_))
    }

    /// Check if result is Err
    pub fn is_err(&self) -> bool {
        matches!(self, ZenResult::Err(_))
    }

    /// Unwrap value or panic
    pub fn unwrap(self) -> T {
        match self {
            ZenResult::Ok(v) => v,
            ZenResult::Err(_) => panic!("called unwrap on Err value"),
        }
    }

    /// Unwrap error or panic
    pub fn unwrap_err(self) -> E {
        match self {
            ZenResult::Ok(_) => panic!("called unwrap_err on Ok value"),
            ZenResult::Err(e) => e,
        }
    }

    /// Map over ok value
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> ZenResult<U, E> {
        match self {
            ZenResult::Ok(v) => ZenResult::Ok(f(v)),
            ZenResult::Err(e) => ZenResult::Err(e),
        }
    }

    /// Flat map over ok value
    pub fn and_then<U, F: FnOnce(T) -> ZenResult<U, E>>(self, f: F) -> ZenResult<U, E> {
        match self {
            ZenResult::Ok(v) => f(v),
            ZenResult::Err(e) => ZenResult::Err(e),
        }
    }

    /// Get ok value or default
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            ZenResult::Ok(v) => v,
            ZenResult::Err(_) => default,
        }
    }

    /// Get ok value or apply function to error
    pub fn unwrap_or_else<F: FnOnce(E) -> T>(self, f: F) -> T {
        match self {
            ZenResult::Ok(v) => v,
            ZenResult::Err(e) => f(e),
        }
    }
}

impl<T: fmt::Display, E: fmt::Display> fmt::Display for ZenResult<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZenResult::Ok(v) => write!(f, "Ok({})", v),
            ZenResult::Err(e) => write!(f, "Err({})", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_ok() {
        let result = ZenResult::Ok(42);
        assert!(result.is_ok());
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_result_err() {
        let result: ZenResult<i32, String> = ZenResult::Err("error".to_string());
        assert!(!result.is_ok());
        assert!(result.is_err());
    }

    #[test]
    fn test_result_map() {
        let result = ZenResult::Ok(5);
        let mapped = result.map(|x| x * 2);
        assert_eq!(mapped.unwrap(), 10);
    }

    #[test]
    fn test_result_and_then() {
        let result = ZenResult::Ok(5);
        let chained = result.and_then(|x| {
            if x > 0 {
                ZenResult::Ok(x * 2)
            } else {
                ZenResult::Err("negative")
            }
        });
        assert_eq!(chained.unwrap(), 10);
    }
}
