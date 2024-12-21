use crate::owl::array::Array;
use crate::owl::node_components::{Meta,MAP_SIZE};


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
