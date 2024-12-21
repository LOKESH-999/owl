#![allow(unused)]

use std::hash::Hash;
use crate::owl::node_components::{DataLine, HashTable,EmptyLine};
pub const NULL_IDX: u16 = 65535; 

pub struct Node<Key:Hash+Ord+PartialOrd+Eq+PartialEq, Val>{
    /// stores the data
    data_line: DataLine<Key,Val>,

    /// Stores empty spaces
    empty_line: EmptyLine,

    /// Store index of relative Entity
    hash_line: HashTable,

    /// Head & Tail of LL for LRU OR MRU
    head:u16,
    tail:u16
}



impl<K,V> Node<K,V>
where K:Hash+Ord+PartialOrd+Eq+PartialEq {
    pub fn insert(&mut self, key:K, value:V){
        unimplemented!()
    }

    pub fn get(&self, key:&K)->Option<V>{
        unimplemented!()
    }

    pub fn delete(&mut self,key:&K)->Option<(K,V)>{
        unimplemented!()
    }
}