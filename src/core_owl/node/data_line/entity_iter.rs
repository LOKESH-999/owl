use super::entity::{Entity, EntityGuard};
use std::{hash::Hash, ptr::NonNull};
use super::super::super::node::NULL_IDX;

/// A custom iterator for traversing `Entity` objects. 
/// This iterator supports two modes:
/// - **Link-based iteration**: Follows the `link` pointers.
/// - **Chain-based iteration**: Follows the `chain` pointers.
///
/// # Type Parameters
/// - `K`: The key type, requiring `Hash`, `Ord`, `PartialOrd`, `Eq`, and `PartialEq`.
/// - `V`: The value type associated with the key.
pub struct EntityIter<K, V>
where
    K: Hash + Ord + PartialOrd + Eq + PartialEq,
{
    /// Raw pointer to the base array of `Entity` objects.
    /// 
    /// # Safety
    /// This is a raw pointer and must be used with caution to avoid undefined behavior.
    ptr: *mut Entity<K, V>,
    
    /// The current index in the iteration.
    /// When `next` equals `NULL_IDX`, the iteration is complete.
    next: u16,
    
    /// Flag indicating the iteration mode:
    /// - `true`: Link-based iteration.
    /// - `false`: Chain-based iteration.
    link: bool,
}

impl<K, V> EntityIter<K, V>
where
    K: Hash + Ord + PartialOrd + Eq + PartialEq,
{
    /// Creates a new `EntityIter`.
    ///
    /// # Arguments
    /// - `ptr`: A `NonNull` pointer to the base array of `Entity` objects.
    /// - `curr`: The starting index for iteration.
    /// - `link`: Determines the iteration mode (`true` for link-based, `false` for chain-based).
    ///
    /// # Returns
    /// - `Ok(EntityIter)`: If the provided pointer is valid.
    /// - `Err(String)`: If the pointer is null.
    pub fn new(ptr: NonNull<Entity<K, V>>, curr: u16, link: bool) -> Result<Self, String> {
        let ptr = ptr.as_ptr();

        // Validate the pointer.
        if ptr.is_null() {
            return Err("Null pointer provided.".to_string());
        }

        // Initialize the iterator with the provided pointer and parameters.
        Ok(EntityIter {
            ptr,       // Pointer to the base array of `Entity` objects.
            next: curr, // Starting index for iteration.
            link,      // Iteration mode.
        })
    }
}

impl<'a, K, V> Iterator for &'a mut EntityIter<K, V>
where
    K: Hash + Ord + PartialOrd + Eq + PartialEq,
{
    /// The type of item returned by the iterator.
    /// Each item is a tuple of:
    /// - `EntityGuard<'a, K, V>`: A locked reference to the current `Entity`.
    /// - `u16`: The index of the current `Entity`.
    type Item = (EntityGuard<'a, K, V>, u16);

    /// Advances the iterator to the next `Entity`.
    ///
    /// # Returns
    /// - `Some((EntityGuard<'a, K, V>, u16))`: If there is a next `Entity`.
    /// - `None`: If the iteration is complete (`next` equals `NULL_IDX`).
    fn next(&mut self) -> Option<Self::Item> {
        // Check if the iteration has reached its end.
        if self.next == NULL_IDX {
            return None;
        }

        unsafe {
            // Get a mutable reference to the current `Entity` and lock it.
            let res = (self.ptr.add(self.next as usize).as_mut().unwrap().lock(), self.next);

            // Update `next` to point to the next `Entity` based on the iteration mode.
            self.next = if self.link {
                res.0.link.next // Use the `link.next` field for link-based iteration.
            } else {
                res.0.chain.next // Use the `chain.next` field for chain-based iteration.
            };

            // Return the locked `Entity` along with its index.
            Some(res)
        }
    }
}
