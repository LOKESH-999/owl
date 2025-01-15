// #! [feature(ptr_as_ref_unchecked)]

use crate::owl::array::Array;
use crate::owl::node_components::{
    Meta,
    MAP_SIZE
};

use std::ops::{
    Index,
    IndexMut
};

pub struct HashTable{
    ///meta data
    meta:Meta,

    /// hash table
    hash:Array<u16>
}


impl HashTable{
    pub fn new()->HashTable{
        let meta = Meta::new();
        let mut arr = Array::<u16>::new(MAP_SIZE);

        /// Initilizing Default Value of 0xFFFFu16
        arr.simd_default(0xFFFF);

        HashTable{
            meta:meta,
            hash:arr
        }
    }
}

impl Index<u16> for HashTable  {
    type Output = u16;
    fn index(&self, index: u16) -> &Self::Output {
        unsafe {
            self.hash.as_ptr().add(index as usize).as_ref_unchecked()
        }
    }
}

impl IndexMut<u16> for HashTable {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        unsafe {
            self.hash.as_ptr().add(index as usize).as_mut_unchecked()
        }
    }
}