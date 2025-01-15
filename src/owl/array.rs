#![allow(unused)]

use std::{
    alloc::{alloc, dealloc, Layout}, // For dynamic memory allocation and deallocation
    mem::{forget, ManuallyDrop},                     // Prevents dropping of ownership
    ptr::NonNull                     // Non-nullable pointer type
};

use std::simd::Simd; // SIMD operations for performance optimization

/// A custom implementation of a dynamically allocated array.
///
/// # Fields
/// * `ptr` - NonNull pointer to the beginning of the array.
/// * `cap` - The capacity of the array.
pub struct Array<T> {
    ptr: NonNull<T>, // Pointer to the allocated memory
    cap: usize       // Capacity of the array
}

impl<T> Array<T> {
    /// Creates a new `Array` with a specified size.
    ///
    /// # Arguments
    /// * `size` - The number of elements to allocate.
    ///
    /// # Returns
    /// A new instance of `Array` with uninitialized memory.
    #[inline]
    pub fn new(size: usize) -> Self {
        // Define the layout for an array of `size` elements of type `T`.
        let layout = Layout::array::<T>(size).unwrap();

        // Allocate the memory block and cast it to a pointer of type `T`.
        let ptr = unsafe { alloc(layout) as *mut T };

        Array {
            ptr: NonNull::new(ptr).unwrap(), // Ensure the pointer is not null
            cap: size
        }
    }
}

/// Provides a default implementation for `Array` where the size is `u16::MAX`.
///
/// This is useful for creating a large array without specifying a size explicitly.
impl<T: Default> Default for Array<T> {
    fn default() -> Self {
        let layout = Layout::array::<T>(0xFFFF).unwrap();
        let ptr = unsafe { alloc(layout) as *mut T };

        Array {
            ptr: NonNull::new(ptr).unwrap(),
            cap: 0xFFFF // Default size is `u16::MAX`
        }
    }
}

impl<T> Array<T> {
    /// Retrieves the value at a specific index.
    ///
    /// # Arguments
    /// * `idx` - Index of the value to retrieve.
    ///
    /// # Returns
    /// The value at the given index.
    #[inline]
    pub unsafe  fn get_unchecked(&self, idx: usize) -> ManuallyDrop<T> {
        unsafe { ((self.ptr.add(idx)).as_ptr() as *mut ManuallyDrop<T>).read() } // Dereference and read the value
    }

    #[inline]
    pub unsafe fn get_mut(&self,idx: usize)->&mut T{
        unsafe {
            self.ptr.add(idx).as_mut()
        }
    }
    /// Sets a value at a specific index.
    ///
    /// # Safety
    /// The caller must ensure the index is within bounds.
    ///
    /// # Arguments
    /// * `data` - The value to write.
    /// * `idx` - The index to write to.
    #[inline]
    pub unsafe fn set_unchecked(&mut self, data: T, idx: usize) {
        self.ptr.add(idx).write(data); // Write the value at the given index
    }

    /// Returns the capacity of the array.
    #[inline]
    pub const fn cap(&self) -> usize {
        self.cap
    }

    /// Consumes the `Array` and returns the raw pointer to the memory block.
    ///
    /// # Safety
    /// The caller is responsible for deallocating the memory.
    pub unsafe fn into_ptr(self) -> *mut T {
        let ptr = self.ptr.as_ptr();
        forget(self); // Prevents the `Drop` trait from being called
        ptr
    }
}

/// Custom implementation of the `Drop` trait for `Array`.
///
/// Ensures the memory allocated is properly deallocated when the `Array` is dropped.
impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        let layout = Layout::array::<T>(self.cap).unwrap();
        let ptr = self.ptr.as_ptr() as *mut u8;
        unsafe { dealloc(ptr, layout) }; // Deallocate the memory block
    }
}

impl<T> Array<T> {
    /// Returns the raw pointer to the memory block.
    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
}

/// Implements default initialization for arrays with SIMD optimization.
///
/// This is specifically designed for types that are compatible with SIMD.
impl<T: std::simd::SimdElement> Array<T> {
    /// Initializes the array using SIMD operations with a specified default value.
    ///
    /// # Arguments
    /// * `default` - The default value to initialize the array with.
    pub fn simd_default(&mut self, default: T) {
        // Create a SIMD value with 32 lanes, all initialized to `default`.
        let simd_val = Simd::from_array([default; 32]);

        // Determine the number of chunks of size 32.
        let chunk = self.cap / 32;
        let ptr = self.ptr.as_ptr();

        // Initialize chunks of 32 elements using SIMD.
        for idx in 0..chunk {
            let simd_ptr = unsafe { ptr.add(idx * 32) as *mut Simd<T, 32> };
            unsafe { simd_ptr.write(simd_val) }; // Write the SIMD value to the chunk
        }

        // Initialize the remaining elements individually.
        for idx in chunk * 32..self.cap {
            unsafe { ptr.add(idx).write(default) };
        }
    }
}
