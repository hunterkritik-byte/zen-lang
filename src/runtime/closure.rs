//! Closure implementation for Zen
//! Supports first-class functions with variable capture and lexical scoping

use std::fmt;

/// Represents a Zen closure with captured environment
#[derive(Clone)]
pub struct ZenClosure {
    /// Parameter names
    pub params: Vec<String>,
    /// Closure body (AST representation)
    pub body: String,
    /// Captured variables from enclosing scope
    pub captures: Vec<(String, String)>, // (name, value)
}

impl ZenClosure {
    /// Create a new closure
    pub fn new(params: Vec<String>, body: String) -> Self {
        ZenClosure {
            params,
            body,
            captures: Vec::new(),
        }
    }

    /// Add captured variable
    pub fn capture(&mut self, name: String, value: String) {
        self.captures.push((name, value));
    }

    /// Get parameters
    pub fn params(&self) -> &[String] {
        &self.params
    }

    /// Get captured variables
    pub fn captures(&self) -> &[(String, String)] {
        &self.captures
    }

    /// Get closure body
    pub fn body(&self) -> &str {
        &self.body
    }

    /// Check if closure accepts given number of arguments
    pub fn accepts_args(&self, arg_count: usize) -> bool {
        self.params.len() == arg_count
    }
}

impl fmt::Debug for ZenClosure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ZenClosure")
            .field("params", &self.params)
            .field("body_length", &self.body.len())
            .field("captures", &self.captures)
            .finish()
    }
}

impl fmt::Display for ZenClosure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<closure({})>", self.params.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_creation() {
        let closure = ZenClosure::new(
            vec!["x".to_string(), "y".to_string()],
            "x + y".to_string(),
        );
        assert_eq!(closure.params.len(), 2);
        assert!(closure.accepts_args(2));
        assert!(!closure.accepts_args(1));
    }

    #[test]
    fn test_closure_capture() {
        let mut closure = ZenClosure::new(
            vec!["x".to_string()],
            "x + multiplier".to_string(),
        );
        closure.capture("multiplier".to_string(), "5".to_string());
        assert_eq!(closure.captures.len(), 1);
    }
}
