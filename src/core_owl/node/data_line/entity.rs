use std::hint::spin_loop; // CPU hint to reduce power consumption during spin-wait loops.
use std::mem::ManuallyDrop; // Prevents automatic cleanup for manual memory management.
use std::hash::Hash; // Enables hashing capabilities for keys.
use std::ops::{Deref, DerefMut}; // Traits for dereferencing and mutable dereferencing.
use std::sync::atomic::{
    AtomicBool, // Atomic boolean for thread-safe lock management.
    Ordering::{Release, Acquire, AcqRel}, // Memory ordering for atomic operations.
};

use super::super::NULL_IDX; // Placeholder for null or sentinel value representation.

/// Represents a doubly linked list node with references to previous and next elements.
#[derive(Debug)]
pub struct Link {
    /// Index of the previous node.
    pub prev: u16,
    /// Index of the next node.
    pub next: u16,
}

/// Provides a default implementation for the `Link` struct.
/// The default `Link` instance has both `prev` and `next` indices set to `NULL_IDX`, 
/// representing a node that is not connected to any other node.
impl Default for Link {
    fn default() -> Self {
        Link {
            prev: NULL_IDX,
            next: NULL_IDX,
        }
    }
}

/// Represents a key-value entity with locking and linking mechanisms.
///
/// # Generics
/// - `K`: Key type, must support hashing, ordering, and equality.
/// - `V`: Value type.
#[derive(Debug)]
pub struct Entity<K, V>
where
    K: Hash + Ord + PartialOrd + Eq + PartialEq,
{
    key: K,              // The key of the entity.
    val: V,              // The value associated with the key.
    lock: AtomicBool,    // Atomic lock to ensure thread safety during access.
    pub link: Link,      // Link for doubly linked list operations.
    pub chain: Link,     // Link for collision handling in hash chains.
}

/// Provides a guard for safely accessing an `Entity` while holding its lock.
///
/// # Lifetimes
/// - `'a`: Lifetime tied to the referenced `Entity`.
#[derive(Debug)]
pub struct EntityGuard<'a, K, V>
where
    K: Hash + Ord + PartialOrd + Eq + PartialEq,
{
    data: &'a mut Entity<K, V>, // Mutable reference to the guarded entity.
}

impl<K, V> Entity<K, V>
where
    K: Hash + Ord + PartialOrd + Eq + PartialEq,
{
    /// Creates a new entity with the given key, value, and links.
    ///
    /// # Arguments
    /// - `key`: The key of the entity.
    /// - `val`: The value associated with the key.
    /// - `link`: Link for the doubly linked list.
    /// - `chain`: Link for hash collision resolution.
    pub fn new(key: K, val: V, link: Link, chain: Link) -> Self {
        Entity {
            key,
            val,
            link,
            chain,
            lock: AtomicBool::new(false), // Initializes the lock to false (unlocked).
        }
    }

    /// Checks if the entity's key matches the given key.
    ///
    /// # Arguments
    /// - `key`: The key to compare against.
    ///
    /// # Returns
    /// `true` if the keys match, otherwise `false`.
    #[inline(always)]
    pub fn is_same_key(&self, key: &K) -> bool {
        &self.key == key
    }

    /// Locks the entity and returns a guard for safe access.
    pub fn lock(&mut self) -> EntityGuard<'_, K, V> {
        while self.lock.swap(true, AcqRel) {
            spin_loop(); // Spin-wait until the lock is acquired.
        }
        EntityGuard { data: self }
    }

    /// Returns a reference to the value stored in the entity.
    #[inline(always)]
    fn as_ref(&self) -> &V {
        &self.val
    }

    /// Sets a new value for the entity.
    ///
    /// # Arguments
    /// - `val`: The new value to set.
    #[inline(always)]
    fn set_val(&mut self, val: V) {
        self.val = val;
    }
}

impl<'a, K, V> EntityGuard<'a, K, V>
where
    K: Hash + Ord + PartialOrd + Eq + PartialEq,
{
    /// Sets a new value for the guarded entity.
    ///
    /// # Arguments
    /// - `val`: The new value to set.
    pub fn set_val(&mut self, val: V) {
        self.data.set_val(val);
    }

    /// Returns a reference to the value of the guarded entity.
    pub fn as_ref(&self) -> &V {
        self.data.as_ref()
    }
}

impl<'a, K, V> Drop for EntityGuard<'a, K, V>
where
    K: Hash + Ord + PartialOrd + Eq + PartialEq,
{
    /// Releases the lock when the guard goes out of scope.
    fn drop(&mut self) {
        self.data.lock.store(false, Release);
    }
}

impl<'a, K, V> Deref for EntityGuard<'a, K, V>
where
    K: Hash + Ord + PartialOrd + Eq + PartialEq,
{
    type Target = Entity<K, V>;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, K, V> DerefMut for EntityGuard<'a, K, V>
where
    K: Hash + Ord + PartialOrd + Eq + PartialEq,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}
