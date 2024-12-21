use std::hash::Hash;
use crate::owl::array::Array;
use crate::owl::node_components::Entity;

pub struct DataLine<Key:Hash+Ord+PartialOrd+Eq+PartialEq,Val>{
    data:Array<Entity<Key,Val>>
}


impl<K,V> DataLine<K,V> 
where K:Hash+Ord+PartialOrd+Eq+PartialEq{
    
    pub fn ptr(&mut self)->*mut Entity<K,V>{
        self.data.as_ptr() 
    }
}