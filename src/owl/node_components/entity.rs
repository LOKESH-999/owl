#![allow(unused)]

use std::hash::Hash;
use crate::owl::node_components::Link;

pub const NULL_IDX: u16 = 65535; 

pub struct Entity<K,V>
where K:Hash+Ord+PartialOrd+Eq+PartialEq{
    /// Key
    key:K,

    /// Value
    value:V,
    
    /// Link for LRU or MRU Cache
    pub link:Link,
    
    /// Chaining if collusion Occure
    pub chain:Link
}

impl<K:Hash+Ord+PartialOrd+Eq+PartialEq,V> Entity<K,V> {

    pub const fn new(key:K,value:V,link:Link,chain:Link)->Self{
        Entity{
            key,
            value,
            link,
            chain
        }
    }

    #[inline]
    pub fn update_link(&mut self,t_prev:u16, t_next:u16)->(u16,u16){
        /// Loading Prev 
        let prev = self.link.prev;

        // Updating Prev as `t_prev`
        self.link.prev = t_prev;

        /// Loading Next
        let next = self.link.next;

        // Updating Next as `t_next`
        self.link.next = t_next;

        /// Returning Previous values of `prev` and `next` before updating
        return (prev,next);
    }

    #[inline]
    pub fn update_chain(&mut self,t_next:u16,t_prev:u16)-> (u16, u16){
        /// Loading Prev 
        let prev = self.chain.prev;

        // Updating Prev as `t_prev`
        self.chain.prev = t_prev;

        /// Loading Next
        let next = self.chain.next;

        // Updating Next as `t_next`
        self.chain.next = t_next;

        /// Returning Previous values of `prev` and `next` before updating
        return (prev,next);
    }

    #[inline]
    pub fn read(&self)->&V{
        &self.value
    }

    #[inline]
    pub fn as_ptr(&mut self) -> *mut Self{
        self as *mut Self
    }

    // #[inline]
    // pub fn link_nodes(mut lhs:&mut Self,mut rhs:&mut Self){
    //     // lhs.chain.next=
    //     todo!()
    // }
}


// impl<K:Hash,V> Drop for Entity<K,V> {
//     fn drop(&mut self) {

//         unimplemented!("link update");
//         unimplemented!("chain update");
//     }    
// }