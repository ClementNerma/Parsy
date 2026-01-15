use std::{
    collections::{BTreeSet, HashSet},
    hash::Hash,
};

/// A container to collect values into
///
/// See [`collect`](`crate::ParserConstUtils::collect`)
pub trait Container<T> {
    /// Create a container
    fn create() -> Self;

    /// Create a container with the provided capacity reserved ahead-of-time
    fn with_capacity(size: usize) -> Self;

    /// Create a container from the provided iterator
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self;

    /// Push a new value into the container
    fn push(&mut self, value: T);
}

impl<T> Container<T> for Vec<T> {
    fn create() -> Self {
        vec![]
    }

    fn with_capacity(size: usize) -> Self {
        Vec::with_capacity(size)
    }

    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().collect()
    }

    fn push(&mut self, value: T) {
        Vec::push(self, value);
    }
}

impl<T: Eq + Hash> Container<T> for HashSet<T> {
    fn create() -> Self {
        HashSet::new()
    }

    fn with_capacity(size: usize) -> Self {
        HashSet::with_capacity(size)
    }

    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().collect::<HashSet<_>>()
    }

    fn push(&mut self, value: T) {
        HashSet::insert(self, value);
    }
}

impl<T: Eq + Ord + Hash> Container<T> for BTreeSet<T> {
    fn create() -> Self {
        BTreeSet::new()
    }

    fn with_capacity(_: usize) -> Self {
        // It is not possible to create a BTreeSet with a provisioned storage
        BTreeSet::new()
    }

    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().collect::<BTreeSet<_>>()
    }

    fn push(&mut self, value: T) {
        BTreeSet::insert(self, value);
    }
}

impl Container<char> for String {
    fn create() -> Self {
        String::new()
    }

    fn with_capacity(size: usize) -> Self {
        String::with_capacity(size)
    }

    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        iter.into_iter().collect()
    }

    fn push(&mut self, value: char) {
        self.push(value)
    }
}

impl Container<String> for String {
    fn create() -> Self {
        String::new()
    }

    fn with_capacity(size: usize) -> Self {
        String::with_capacity(size)
    }

    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        iter.into_iter().collect()
    }

    fn push(&mut self, value: String) {
        self.push_str(&value)
    }
}

/// A container that discards every value
///
/// Does not allocate any memory
#[derive(Clone, Copy)]
pub struct NoAllocContainer;

impl<T> Container<T> for NoAllocContainer {
    fn create() -> Self {
        Self
    }

    fn with_capacity(_: usize) -> Self {
        Self
    }

    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        // Trigger the iterator to trigger panics etc.
        iter.into_iter().count();
        Self
    }

    fn push(&mut self, _: T) {}
}
