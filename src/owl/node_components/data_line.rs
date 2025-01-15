use std::hash::Hash;
use crate::owl::array::Array;
use crate::owl::node::NULL_IDX;
use crate::owl::node_components::Entity;
use std::mem::ManuallyDrop;

use super::Link;

pub struct DataLine<Key:Hash+Ord+PartialOrd+Eq+PartialEq+Clone,Val:Clone>{
    data:Array<Entity<Key,Val>>
}


impl<K,V> DataLine<K,V> 
where K:Hash+Ord+PartialOrd+Eq+PartialEq+Clone,V:Clone{
    pub fn new()->Self{
        let array = Array::<Entity<K,V>>::new(65523);
        Self { data: array }
    }

    pub fn ptr(&mut self)->*mut Entity<K,V>{
        self.data.as_ptr() 
    }

    pub fn read(&self,idx:u16)->ManuallyDrop<Entity<K,V>>{
        unsafe {
            self.data.get_unchecked(idx as usize)
        }
    }

    pub unsafe fn set(&mut self,key:K,value:V,idx:u16,link:Link,chain:Link){
        let data = Entity::new(key, value, idx, link, chain);
        self.data.set_unchecked(data, idx as usize);
    }

    pub fn get_mut(&self,idx:u16)->&mut Entity<K,V>{
        unsafe {
            self.data.get_mut(idx as usize)   
        }
    }

    pub fn get_ref(&self,idx:u16)->&Entity<K,V>{
        unsafe {
            self.data.get_mut(idx as usize)
        }
    }

}