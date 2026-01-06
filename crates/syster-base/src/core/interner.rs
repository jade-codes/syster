//! String interner for efficient string storage and comparison.
//!
//! Uses `Rc<str>` for cheap cloning (reference count increment instead of allocation).
//! The interner deduplicates strings so identical strings share the same allocation.

use std::collections::HashSet;
use std::rc::Rc;

/// An interned string - cheap to clone (just Rc increment)
pub type IStr = Rc<str>;

/// String interner that deduplicates strings.
///
/// Interning a string returns an `Rc<str>` that can be cheaply cloned.
/// If the same string is interned multiple times, the same `Rc` is returned.
#[derive(Debug, Default, Clone)]
pub struct Interner {
    strings: HashSet<Rc<str>>,
}

impl Interner {
    pub fn new() -> Self {
        Self::default()
    }

    /// Intern a string, returning a cheap-to-clone reference.
    ///
    /// If the string was already interned, returns the existing `Rc`.
    /// Otherwise, creates a new `Rc` and stores it.
    pub fn intern(&mut self, s: &str) -> IStr {
        if let Some(existing) = self.strings.get(s) {
            Rc::clone(existing)
        } else {
            let rc: Rc<str> = Rc::from(s);
            self.strings.insert(Rc::clone(&rc));
            rc
        }
    }

    /// Intern an owned string, avoiding allocation if possible.
    pub fn intern_string(&mut self, s: String) -> IStr {
        if let Some(existing) = self.strings.get(s.as_str()) {
            Rc::clone(existing)
        } else {
            let rc: Rc<str> = Rc::from(s);
            self.strings.insert(Rc::clone(&rc));
            rc
        }
    }

    /// Get an interned string if it exists, without creating it.
    pub fn get(&self, s: &str) -> Option<IStr> {
        self.strings.get(s).cloned()
    }

    /// Number of unique strings interned.
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    /// Returns true if no strings have been interned.
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }

    /// Clear all interned strings.
    pub fn clear(&mut self) {
        self.strings.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intern_returns_same_rc() {
        let mut interner = Interner::new();
        let a = interner.intern("hello");
        let b = interner.intern("hello");
        assert!(Rc::ptr_eq(&a, &b));
    }

    #[test]
    fn test_intern_different_strings() {
        let mut interner = Interner::new();
        let a = interner.intern("hello");
        let b = interner.intern("world");
        assert!(!Rc::ptr_eq(&a, &b));
        assert_eq!(&*a, "hello");
        assert_eq!(&*b, "world");
    }

    #[test]
    fn test_clone_is_cheap() {
        let mut interner = Interner::new();
        let a = interner.intern("test");
        let b = a.clone(); // Just increments ref count
        assert!(Rc::ptr_eq(&a, &b));
        assert_eq!(Rc::strong_count(&a), 3); // interner + a + b
    }

    #[test]
    fn test_get_existing() {
        let mut interner = Interner::new();
        interner.intern("exists");
        assert!(interner.get("exists").is_some());
        assert!(interner.get("missing").is_none());
    }
}
