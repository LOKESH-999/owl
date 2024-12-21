#![allow(unused)]

use std::{alloc::{alloc, dealloc, Layout}, mem::forget, ptr::NonNull};

use std::simd::Simd;

pub struct Array<T>{
    ptr:NonNull<T>,
    cap:usize
}

impl<T> Array<T>{
    /// impl with custom size 
    #[inline]
    pub fn new(size:usize) -> Self{
        let layout = Layout::array::<T>(size).unwrap();
        let ptr = unsafe {
            alloc(layout) as *mut T   
            };
        Array{
          ptr :NonNull::new(ptr).unwrap(),
          cap :size
        }
    }
}

/// default of size ```{u16::MAX}```
impl<T:Default> Default for Array<T>{
    fn default() -> Self {
        let layout = Layout::array::<T>(0xFFFF).unwrap();
        let ptr = unsafe {
            alloc(layout) as *mut T
        };
        Array { 
            ptr : NonNull::new(ptr).unwrap(),
            cap : 0xFFFFF
        }
    }
}


impl <T> Array<T> {
    #[inline]
    pub fn get(&mut self,idx:usize) -> T{
        unsafe {
            self.ptr.add(idx).read()
        }
    }

    #[inline]
    pub unsafe fn set(&mut self,data:T,idx:usize){
        unsafe {
            self.ptr.add(idx).write(data);
        }
    }

    pub fn into_ptr(self)->*mut T{
        let ptr = self.ptr.as_ptr();
        forget(self);
        ptr
    }
}

impl<T> Drop for Array<T>{
    fn drop(&mut self) {
        let layout = Layout::array::<T>(self.cap).unwrap();
        let ptr = self.ptr.as_ptr() as  *mut u8;
        unsafe {
            dealloc(ptr, layout);
        }
    }
}

impl<T> Array<T> {
    pub fn as_ptr(&mut self) -> *mut T{
        self.ptr.as_ptr()
    }
}


/// impl simd default
impl<T: std::simd::SimdElement> Array<T>{
    pub fn simd_default(&mut self, default:T){

        /// Initilizing SIMD Values with 0xFFFF as default
        let simd_val = Simd::from_array([default;32]);
        
        /// Chunk size of 32 blocks
        let chunk = self.cap/32;

        let ptr = self.ptr.as_ptr();

        for idx in 0..chunk{
            /// Pointer Type Casting
            let simd_ptr = unsafe {
                ptr.add(idx*32) as *mut Simd<T,32>
            };

            /// Writing Default value to HashTable Array
            unsafe {
                simd_ptr.write(simd_val);   
            }
        }

        /// Writing to remainig elements 
        for idx in chunk*32..self.cap{
            unsafe {
                ptr.add(idx).write(default);
            }
        }
    }
}