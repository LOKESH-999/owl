#![allow(unused)]

use std::mem::ManuallyDrop;
// Importing the custom `Array` implementation
use crate::owl::array::Array;

use std::sync::atomic::{
    AtomicUsize,
    Ordering::{
        Relaxed,
        SeqCst
    }
};

/// Represents a line to store the indices of empty blocks.
/// This enables fast allocation of memory blocks when needed.
pub struct EmptyLine {
    // Array to store the indices of empty blocks.
    arr: Array<u16>,

    // Number of empty blocks currently available.
    len: AtomicUsize,
}

impl EmptyLine {
    /// Creates a new `EmptyLine` with a specified size.
    /// 
    /// # Arguments
    /// * `size` - The size of the array to hold empty block indices.
    ///
    /// # Returns
    /// A new `EmptyLine` instance with an empty array and size.
    pub fn new(size: usize) -> Self {
        EmptyLine {
            arr: Array::new(size), // Initialize the array with the given size
            len: AtomicUsize::new(size),            // Set the size of the array
        }
    }

    /// Initializes the empty indices in sequential order.
    /// 
    /// This method populates the `arr` with sequential values 
    /// from `0` to `len - 1`. These values represent the indices
    /// of the empty blocks in the `DataLine`.
    pub fn init(&mut self) {
        // Populate the array with sequential values
        for value in 0..self.len.load(Relaxed) {
            unsafe {
                // Set each index with the value itself
                self.arr.set_unchecked(value as u16, value);
            }
        }
    }
}

impl EmptyLine {
    /// Pushes a value to the array without bounds checking.
    ///
    /// # Safety
    /// The caller must ensure that the capacity of the array is not exceeded.
    pub unsafe fn push_unchecked(&mut self, data: u16) ->u16{
        // Write the value at the current length index.
        self.arr.set_unchecked(data, self.len.load(SeqCst));
        // Increment the length to reflect the new size.
        self.len.fetch_add(1, SeqCst) as u16
    }

    /// Pushes a value to the array with bounds checking.
    ///
    /// # Arguments
    /// * `data` - The value to push.
    ///
    /// # Returns
    /// * `Some(index)` - If the value was successfully pushed, returns the index of the inserted value.
    /// * `None` - If the array is full.
    pub fn push(&mut self, data: u16) -> Option<u16> {
        // Ensure there's enough capacity to add a new element.
        if self.len.load(Relaxed) >= self.arr.cap() {
            return None;
        }

        // Safely push the value.
        return Some(unsafe { self.push_unchecked(data)}) // Return the index where the value was inserted.
    }

    /// Pops the last value from the array.
    ///
    /// # Returns
    /// * The last value in the array.
    ///
    /// # Safety
    /// Caller must ensure the array is not empty before calling this function.
    pub fn pop(&mut self) -> Option<ManuallyDrop<u16>> {
        match self.len.load(SeqCst) {
            0 =>{ return None;},
            x =>{
                let result = unsafe { self.arr.get_unchecked(x) };
                self.len.fetch_sub(1, SeqCst);
                return Some(result);
            }
        }
    }
}
