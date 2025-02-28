use std::{
    alloc::{alloc, dealloc, Layout}, // For dynamic memory allocation and deallocation.
    mem::{forget, ManuallyDrop},     // `forget` prevents dropping of ownership, `ManuallyDrop` for manual drop control.
    ptr::NonNull                     // Non-nullable pointer type.
};

use std::simd::Simd; // SIMD operations for performance optimization.

use super::ARR_SIZE; // Importing a constant defining the fixed array size.

/// A structure representing an unsafe dynamically allocated array.
/// 
/// # Safety
/// - This structure utilizes raw pointers and manual memory management.
/// - Users must ensure safety while using methods to prevent undefined behavior.
pub struct UnsafeArray<T> {
    /// A non-nullable pointer to the start of the allocated memory block.
    ptr: NonNull<T>,
}

impl<T> UnsafeArray<T> {
    /// Creates a new `UnsafeArray` instance with a predefined size.
    /// 
    /// # Arguments
    /// * `size` - The number of elements to allocate.
    /// 
    /// # Panics
    /// This function will panic if the `size` does not match the predefined `ARR_SIZE`.
    #[inline]
    pub fn new(size: usize) -> Self {
        assert_eq!(size, ARR_SIZE as usize); // Ensure the requested size matches the fixed size.
        let layout = Layout::array::<T>(size).unwrap(); // Create a memory layout for the array.
        let ptr = unsafe {
            // Allocate memory and create a non-null pointer.
            NonNull::new(alloc(layout) as *mut T).unwrap()
        };
        UnsafeArray { ptr }
    }
}

impl<T> UnsafeArray<T> {
    /// Provides manual ownership of the element at the specified index as a `ManuallyDrop`.
    /// 
    /// # Arguments
    /// * `idx` - The index of the element.
    /// 
    /// # Returns
    /// A `ManuallyDrop<T>` instance for the specified element.
    /// 
    /// # Safety
    /// Caller must ensure the index is within bounds to avoid undefined behavior.
    #[inline]
    pub fn as_manual_cp(&self, idx: usize) -> ManuallyDrop<T> {
        unsafe { ((self.ptr.add(idx)).as_ptr() as *mut ManuallyDrop<T>).read() }
    }

    /// Provides a mutable reference to the element at the specified index.
    /// 
    /// # Arguments
    /// * `idx` - The index of the element.
    /// 
    /// # Returns
    /// A mutable reference to the element.
    /// 
    /// # Safety
    /// Caller must ensure the index is within bounds to avoid undefined behavior.
    #[inline]
    pub fn as_mut(&self, idx: usize) -> &mut T {
        unsafe { self.ptr.add(idx).as_mut() }
    }

    /// Provides an immutable reference to the element at the specified index.
    /// 
    /// # Arguments
    /// * `idx` - The index of the element.
    /// 
    /// # Returns
    /// An immutable reference to the element.
    /// 
    /// # Safety
    /// Caller must ensure the index is within bounds to avoid undefined behavior.
    #[inline]
    pub fn as_ref(&self, idx: usize) -> &T {
        unsafe { self.ptr.add(idx).as_ref() }
    }
}

impl<T> UnsafeArray<T> {
    /// Sets the value of the element at the specified index.
    /// 
    /// # Arguments
    /// * `idx` - The index of the element.
    /// * `data` - The value to set at the specified index.
    /// 
    /// # Safety
    /// Caller must ensure the index is within bounds to avoid undefined behavior.
    #[inline(always)]
    pub fn set(&mut self, idx: usize, data: T) {
        unsafe {
            self.ptr.add(idx).write(data); // Write data to the specified index.
        }
    }

    pub fn as_ptr(&mut self)->*mut T{
        self.ptr.as_ptr()
    }
}

impl<T> Drop for UnsafeArray<T> {
    /// Drops the `UnsafeArray` and deallocates its memory.
    /// 
    /// # Safety
    /// Ensures that the memory layout matches the allocation to avoid undefined behavior.
    fn drop(&mut self) {
        let layout = Layout::array::<T>(ARR_SIZE as usize).unwrap(); // Get the memory layout.
        unsafe {
            dealloc(self.ptr.as_ptr() as *mut u8, layout); // Deallocate memory.
        }
    }
}


/// Implements default initialization for arrays with SIMD optimization.
///
/// This is specifically designed for types that are compatible with SIMD.
impl<T: std::simd::SimdElement> UnsafeArray<T> {
    /// Initializes the array using SIMD operations with a specified default value.
    /// # Arguments
    /// * `default` - The default value to initialize the array with.
    /// * `size` - The size of the array.
    pub fn simd_default(default: T, size: usize) -> Self {
        // Determine the number of elements per SIMD chunk dynamically
        const SIMD_SIZE: usize = 32;
        let simd_val = Simd::splat(default); // Initialize SIMD value with `default`
        let chunk_count = size / SIMD_SIZE;
        
        let mut arr = Self::new(size);
        let ptr = arr.as_ptr();

        // Initialize chunks of SIMD_SIZE elements using SIMD
        for idx in 0..chunk_count {
            let simd_ptr = unsafe { ptr.add(idx * SIMD_SIZE) as *mut Simd<T, SIMD_SIZE> };
            unsafe { simd_ptr.write(simd_val) }; // Write SIMD value to the chunk
        }

        // Initialize the remaining elements individually.
        for idx in chunk_count * SIMD_SIZE..size {
            unsafe { ptr.add(idx).write(default) };
        }

        arr
    }
}
