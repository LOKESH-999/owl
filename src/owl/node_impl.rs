#![allow(dead_code)]

use std::hash::Hash;

pub trait NodeImpl<K: Hash + Ord + PartialOrd + Eq + PartialEq, V> {
    /// Inserts a key-value pair into the node.
    fn insert(&mut self, key: K, value: V);

    /// Retrieves the value associated with the given key.
    /// Assumes the key exists; otherwise, this might panic or result in undefined behavior.
    fn get(&mut self, key: &K) -> V;

    /// Deletes the key-value pair associated with the given key.
    /// Returns an `Option` containing the removed key-value pair if it existed.
    fn delete(&mut self, key: &K) -> Option<(K, V)>;
}
