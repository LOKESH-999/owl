#![allow(unused)]

use std::hint::spin_loop;
use std::mem::ManuallyDrop;
use std::hash::Hash;
use std::ops::{
    Deref,
    DerefMut
};
use std::sync::atomic::{
    AtomicBool,
    Ordering::{
        Release,
        Acquire
    }
};
use crate::owl::node_components::Link;

use crate::owl::node::NULL_IDX;

pub struct EntityGuard<'a,K,V>
where K:Hash+Ord+PartialOrd+Eq+PartialEq+Clone,V:Clone{
    data:&'a mut Entity<K,V>
}

/// ### TODO convert SpinLock to RWlock

pub struct Entity<K,V>
where K:Hash+Ord+PartialOrd+Eq+PartialEq+Clone,V:Clone{
    /// Key
    key:K,

    /// Value
    value:V,

    /// /// Index of the entity in a DataLine.
    idx:u16,

    /// Lock to manage concurrent access.
    lock:AtomicBool,

    /// Link for LRU or MRU Cache.
    pub link:Link,
    
    /// Link for handling hash collisions.
    pub chain:Link
}

impl<K:Hash+Ord+PartialOrd+Eq+PartialEq+Clone,V:Clone> Entity<K,V> {

    pub const fn new(key:K,value:V,idx:u16,link:Link,chain:Link)->Self{
        let lock = AtomicBool::new(false);
        Entity{
            key,
            value,
            lock,
            idx,
            link,
            chain
        }
    }

    #[inline]
    fn update_link(&mut self,t_prev:u16, t_next:u16)->(u16,u16){
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
    fn update_chain(&mut self,t_next:u16,t_prev:u16)-> (u16, u16){
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
    fn read(&self)->ManuallyDrop<V>{
        unsafe {
            (&self.value as *const V as *const ManuallyDrop<V>).read()
        }
    }

    #[inline]
    fn as_ptr(&mut self) -> *mut Self{
        self as *mut Self
    }

    #[inline]
    fn idx(&self)->u16{
        self.idx
    }
    
    // pub fn read_lock(&self)

    pub fn lock(&mut self)->EntityGuard<'_,K,V>{
        while self.lock.swap(true, Acquire) {
            spin_loop();
        }
        EntityGuard { data: self }
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


impl<K,V> Deref for EntityGuard<'_,K,V>
where K:Hash+Ord+PartialOrd+Eq+PartialEq+Clone,V:Clone{
    type Target = Entity<K,V>;
    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<K,V> DerefMut for EntityGuard<'_,K,V> 
where K:Hash+Ord+PartialOrd+Eq+PartialEq+Clone,V:Clone{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

impl<K,V> Drop for EntityGuard<'_,K,V>
where K:Hash+Ord+PartialOrd+Eq+PartialEq+Clone,V:Clone {
    fn drop(&mut self) {
        self.lock.store(false, Release);
    }
}