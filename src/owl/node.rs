#![allow(unused)]

use std::hash::Hash;
use crate::owl::node_components::{
    DataLine, 
    HashTable, 
    EmptyLine
};

use super::{
    node_components::{Entity, Link},
    node_impl::NodeImpl
};
pub const NULL_IDX: u16 = 65535; 
use crate::owl::node_components::MAP_SIZE;
use std::sync::atomic::{
    AtomicU16,
    Ordering::Relaxed
};

pub struct Node<Key:Hash+Ord+PartialOrd+Eq+PartialEq+Clone, Val:Clone>{
    /// stores the data
    data_line: DataLine<Key,Val>,

    /// Stores empty spaces
    empty_line: EmptyLine,

    /// Store index of relative Entity
    hash_line: HashTable,

    /// Head & Tail of LL for LRU OR MRU
    head:AtomicU16,
    tail:AtomicU16
}



impl<K,V> Node<K,V>
where K:Hash+Ord+PartialOrd+Eq+PartialEq+Clone,V:Clone{
    pub fn new()->Self{
        let mut empty_line = EmptyLine::new(u16::MAX as usize);
        empty_line.init();
        Self { 
            data_line: DataLine::new(), 
            empty_line, 
            hash_line: HashTable::new(), 
            head: AtomicU16::new(NULL_IDX), 
            tail: AtomicU16::new(NULL_IDX) 
        }
    }
}

impl<K,V> NodeImpl<K,V> for Node<K,V>
where K:Hash+Ord+PartialOrd+Eq+PartialEq+Clone,V:Clone{
    fn delete(&mut self, key: &K) -> Option<(K, V)> {
        todo!()
    }
    fn get(&mut self, key: &K) -> Option<V> {
        todo!()
    }
    fn insert(&mut self,hash_val:u64, key: K, value: V) {
        let hash_idx = hash_val as usize % MAP_SIZE;
        let new_idx = match self.empty_line.pop(){
            Some(idx) => idx ,
            None=>{
                unimplemented!("need to implement")
            }
        };
        let idx  = unsafe { std::mem::transmute(new_idx) };
        let link = Link::default();
        let chain = Link::default();
        unsafe {self.data_line.set(key, value, idx, link, chain);}
        match  self.hash_line[hash_idx as u16] {
            NULL_IDX =>{
                self.hash_line[hash_idx as u16] = idx
            }
            else_=>{
                todo!("collusion occoured need to implement chain process")
            }
        }
    }
}