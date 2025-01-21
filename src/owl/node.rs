#![allow(unused)]


// Importing required traits and modules
use std::{hash::Hash, mem::ManuallyDrop};
use crate::owl::node_components::{
    DataLine, 
    HashTable, 
    EmptyLine
};

// Importing additional node components and node implementation traits
use super::{
    node_components::{Entity, Link},
    node_impl::NodeImpl
};

// Define a constant for a null index
pub const NULL_IDX: u16 = 65535; 

use crate::owl::node_components::MAP_SIZE;
use std::sync::atomic::{
    AtomicU16,
    Ordering::{
        Relaxed,
        Acquire,
        Release
    }
};

/// A `Node` represents a caching mechanism using data lines, 
/// hash tables, and an empty line for space management.
/// Generic parameters:
/// - `Key`: Must implement `Hash`, `Ord`, `PartialOrd`, `Eq`, and `Clone`.
/// - `Val`: Must implement `Clone`.
pub struct Node<Key:Hash+Ord+PartialOrd+Eq+PartialEq+Clone, Val:Clone> {
    /// Stores the data in key-value pairs
    data_line: DataLine<Key,Val>,

    /// Manages empty spaces for efficient allocation
    empty_line: EmptyLine,

    /// Maps hashes to indices in the `data_line`
    hash_line: HashTable,

    /// Head of the linked list for LRU or MRU management
    head: AtomicU16,

    /// Tail of the linked list for LRU or MRU management
    tail: AtomicU16
}

impl<K, V> Node<K, V>
where 
    K: Hash + Ord + PartialOrd + Eq + PartialEq + Clone,
    V: Clone,
{
    /// Creates a new `Node` instance with initialized components
    pub fn new() -> Self {
        let mut empty_line = EmptyLine::new(u16::MAX as usize);
        empty_line.init(); // Initialize the empty line to mark all indices as available
        Self { 
            data_line: DataLine::new(), 
            empty_line, 
            hash_line: HashTable::new(), 
            head: AtomicU16::new(NULL_IDX), // Start with null index for LRU/MRU
            tail: AtomicU16::new(NULL_IDX)  // Start with null index for LRU/MRU
        }
    }
}

// Implementing the `NodeImpl` trait for the `Node` struct
impl<K, V> NodeImpl<K, V> for Node<K, V>
where 
    K: Hash + Ord + PartialOrd + Eq + PartialEq + Clone,
    V: Clone,
{
    /// Deletes a key-value pair from the node. To be implemented.
    fn delete(&mut self, key: &K) -> Option<(K, V)> {
        todo!() // Placeholder for future implementation
    }

    /// Retrieves the value for a given key from the node. To be implemented.
    fn get(&mut self, key: &K) -> Option<V> {
        todo!() // Placeholder for future implementation
    }

    /// Inserts a key-value pair into the node.
    /// Handles hash collisions and manages the empty space efficiently.
    fn insert(&mut self, hash_val: u64, key: K, value: V) {
        // Compute the index in the hash table
        let hash_idx = hash_val as usize % MAP_SIZE;

        // Retrieve a new index from the empty line
        let new_idx = match self.empty_line.pop() {
            Some(idx) => idx, // Use the available index
            None => {
                let tail = self.tail.load(Acquire);
                let data = self.data_line.get_mut(tail);
                let prev = data.link.prev;
                todo!("implement auto chain and auto link");
                self.tail.store(prev, Release);
                ManuallyDrop::new(tail)
            }
        };

        // Safely convert the new index into a valid type
        let idx = unsafe { std::mem::transmute(new_idx) };

        // Default values for link and chain components
        let link = Link::default();
        let chain = Link::default();

        // Insert the key-value pair into the data line
        unsafe {
            self.data_line.set(key, value, idx, link, chain);
        }

        // Check if there's already a hash collision
        match self.hash_line[hash_idx as u16] {
            NULL_IDX => {
                // No collision, directly map the hash index
                self.hash_line[hash_idx as u16] = idx;
            }
            _ => {
                // Collision occurred, chain process needs implementation
                todo!("collusion occurred, need to implement chain process");
            }
        }
    }
}
