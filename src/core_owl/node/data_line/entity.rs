
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
pub struct Link {
    /// Index of the previous node.
    pub prev: u16,
    /// Index of the next node.
    pub next: u16,
}

/// Provides a default implementation for the `Link` struct.
///
/// The default `Link` instance has both `prev` and `next` indices set to `NULL_IDX`. 
/// This represents a node that is not connected to any other node.
///
/// # Examples
/// ```
/// use your_module::{Link, NULL_IDX};
///
/// let default_link = Link::default();
/// assert_eq!(default_link.prev, NULL_IDX);
/// assert_eq!(default_link.next, NULL_IDX);
/// ```
impl Default for Link {
    fn default() -> Self {
        Link {
            /// The default value for `prev` is set to `NULL_IDX`, indicating no previous connection.
            prev: NULL_IDX,
            
            /// The default value for `next` is set to `NULL_IDX`, indicating no next connection.
            next: NULL_IDX,
        }
    }
}

/// Represents a key-value entity with locking and linking mechanisms.
///
/// # Generics
/// - `K`: Key type, must support hashing, ordering, and equality.
/// - `V`: Value type.
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
    ///
    /// # Examples
    /// ```
    /// let entity = Entity::new(
    ///     "key1".to_string(),
    ///     42,
    ///     Link { prev: 0, next: 0 },
    ///     Link { prev: 0, next: 0 },
    /// );
    /// ```
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
    ///
    /// # Examples
    /// ```
    /// let entity = Entity::new(
    ///     "key1".to_string(),
    ///     42,
    ///     Link { prev: 0, next: 0 },
    ///     Link { prev: 0, next: 0 },
    /// );
    /// assert!(entity.is_same_key(&"key1".to_string()));
    /// ```
    pub fn is_same_key(&self, key: &K) -> bool {
        &self.key == key
    }

    /// Locks the entity and returns a guard for safe access.
    ///
    /// # Examples
    /// ```
    /// let mut entity = Entity::new(
    ///     "key1".to_string(),
    ///     42,
    ///     Link { prev: 0, next: 0 },
    ///     Link { prev: 0, next: 0 },
    /// );
    /// let guard = entity.lock();
    /// assert_eq!(*guard.as_ref(), 42);
    /// ```
    pub fn lock(&mut self) -> EntityGuard<'_, K, V> {
        while self.lock.swap(true, AcqRel) {
            spin_loop(); // Spin-wait until the lock is acquired.
        }
        EntityGuard { data: self }
    }

    /// Returns a reference to the value stored in the entity.
    ///
    /// # Examples
    /// ```
    /// let entity = Entity::new(
    ///     "key1".to_string(),
    ///     42,
    ///     Link { prev: 0, next: 0 },
    ///     Link { prev: 0, next: 0 },
    /// );
    /// assert_eq!(entity.as_ref(), &42);
    /// ```
    fn as_ref(&self) -> &V {
        &self.val
    }

    /// Sets a new value for the entity.
    ///
    /// # Arguments
    /// - `val`: The new value to set.
    ///
    /// # Examples
    /// ```
    /// let mut entity = Entity::new(
    ///     "key1".to_string(),
    ///     42,
    ///     Link { prev: 0, next: 0 },
    ///     Link { prev: 0, next: 0 },
    /// );
    /// entity.set_val(84);
    /// assert_eq!(entity.as_ref(), &84);
    /// ```
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
    ///
    /// # Examples
    /// ```
    /// let mut entity = Entity::new(
    ///     "key1".to_string(),
    ///     42,
    ///     Link { prev: 0, next: 0 },
    ///     Link { prev: 0, next: 0 },
    /// );
    /// let mut guard = entity.lock();
    /// guard.set_val(84);
    /// assert_eq!(guard.as_ref(), &84);
    /// ```
    pub fn set_val(&mut self, val: V) {
        self.data.set_val(val);
    }

    /// Returns a reference to the value of the guarded entity.
    ///
    /// # Examples
    /// ```
    /// let mut entity = Entity::new(
    ///     "key1".to_string(),
    ///     42,
    ///     Link { prev: 0, next: 0 },
    ///     Link { prev: 0, next: 0 },
    /// );
    /// let guard = entity.lock();
    /// assert_eq!(guard.as_ref(), &42);
    /// ```
    pub fn as_ref(&self) -> &V {
        self.data.as_ref()
    }
}


impl<'a,K,V> Drop for EntityGuard<'a,K,V>
where K:Hash+Ord+PartialOrd+Eq+PartialEq{
    fn drop(&mut self) {
        self.data.lock.store(false, Release);
    }
}

impl<'a,K,V> Deref for EntityGuard<'a,K,V> 
where K:Hash+Ord+PartialOrd+Eq+PartialEq{
    type Target = Entity<K,V>;
    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a,K,V> DerefMut for EntityGuard<'a,K,V>
where K:Hash+Ord+PartialOrd+Eq+PartialEq{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}