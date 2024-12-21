#![allow(unused)]


 mod empty_line;
 mod link;
 mod hash_store;
 mod entity;
 mod data_line;
use crate::owl::array::Array;
use std::hash::Hash;

/// ReExporting Values
pub const MAP_SIZE:usize = 65521;
pub use empty_line::EmptyLine;
pub use link::Link;
pub use entity::Entity;
pub use hash_store::HashTable;
pub use data_line::DataLine;

pub struct Meta{
    /// Store no of reads
    pub reads:u32,

    /// Store no of writes
    pub writes:u32,

    /// Store no of unsets
    pub unset:u16,

    /// Store length of Array
    pub len:u16,
}

impl Meta {
    pub fn new()->Meta{
        Meta { reads: 0, writes: (0), unset: (0), len: (0) }
    }
}

