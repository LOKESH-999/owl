use std::ptr::NonNull;

use super::array::{
    ARR_SIZE, // Constant representing the array size
    unsafe_array::UnsafeArray, // Custom UnsafeArray implementation
};

/// A simple `HashLine` implementation using an `UnsafeArray<u16>` for storage.
/// The `HashLine` stores index mappings for hashed values.
pub struct HashLine {
    arr: UnsafeArray<u16>, // Underlying array storing `u16` indices
}

impl HashLine {
    /// Creates a new `HashLine` instance with an `UnsafeArray` of size `ARR_SIZE`.
    pub fn new() -> Self {
        let size = ARR_SIZE as usize; // Convert array size to `usize`
        let arr = UnsafeArray::new(size); // Initialize UnsafeArray with the given size
        HashLine { arr }
    }

    /// Retrieves the index stored at a given position in the `HashLine`.
    ///
    /// # Arguments
    /// * `idx` - The index from which to retrieve the stored value.
    ///
    /// # Returns
    /// * The `u16` index stored at the specified position.
    #[inline(always)]
    pub fn get_idx(&self, idx: usize) -> u16 {
        *self.arr.as_ref(idx) // Retrieve the value at `idx`
    }

    /// Sets the index at a specific position in the `HashLine`.
    ///
    /// # Arguments
    /// * `idx` - The position at which to store the index.
    /// * `val` - The `u16` index value to store.
    #[inline(always)]
    pub fn set_idx(&mut self, idx: usize, val: u16) {
        self.arr.set(idx, val); // Store `val` at `idx`
    }

    /// Sets a default value (`ARR_SIZE`) at a given position in the `HashLine`.
    ///
    /// # Arguments
    /// * `idx` - The position at which to store the default value.
    #[inline(always)]
    pub fn set_default(&mut self, idx: usize) {
        self.arr.set(idx, ARR_SIZE); // Assign `ARR_SIZE` as the default value
    }
}
