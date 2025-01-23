
use std::hash::Hash;
use crate::core_owl::node::data_line::entity::Link;

use super::entity::Entity;
use super::super::array::{
    ARR_SIZE,
    unsafe_array::UnsafeArray
};
pub struct DataLine<K,V>
where K:Hash+Ord+PartialOrd+Eq+PartialEq{
    data:UnsafeArray<Entity<K,V>>
}

impl<K,V> DataLine<K,V>
where K:Hash+Ord+PartialOrd+Eq+PartialEq{
    pub fn new()->Self{
        DataLine{
            data:UnsafeArray::new(ARR_SIZE as usize)
        }
    }
}

use super::data_line_impl::{ChainLinker, DataLineImpl};
use super::entity_iter::EntityIter;
use std::ptr::NonNull;

impl<K,V> DataLineImpl<K,V> for DataLine<K,V>
where K:Hash+Ord+PartialOrd+Eq+PartialEq+Default,V:Default{
    fn as_ptr(&self)->*mut Entity<K,V> {
        self.data.as_mut(0) 
    }
    fn get_ref(&self,idx:u16)->&Entity<K,V> {
        self.data.as_ref(idx as usize)
    }
    fn get_mut(&mut self,idx:u16)->&mut Entity<K,V> {
        self.data.as_mut(idx as usize)
    }
    fn entity_iter(&self,idx:u16,link:bool)->EntityIter<K,V> {
        EntityIter::new(
            NonNull::new(self.as_ptr()).unwrap(), 
            idx,
            link
        ).unwrap()
    }
    fn set_val(&mut self,val:V,idx:u16) {
        self.data.as_mut(idx as usize).lock().set_val(val);
    }
    fn take(&self,key:&K,idx:u16)->Option<Entity<K,V>> {
        for entity in &mut self.entity_iter(idx, false){
            if entity.0.is_same_key(key){
                let r= unsafe {
                    todo!("need to update link connections and chain connections");
                    self.link_cl(self.as_ptr(), entity.0, true);
                    let res = self.as_ptr().add(entity.1 as usize).replace(
                        Entity::new(
                        K::default(), V::default(), Link::default(), Link::default()
                    ));
                    res
                };
                return Some(r);
            }
        }
        None
    }
    fn lock_entity(&self,idx:u16) ->super::entity::EntityGuard<'_,K,V> {
        self.data.as_mut(idx as usize).lock()
    }
}


impl<K,V> ChainLinker for DataLine<K,V>
where K:Hash+Ord+PartialOrd+Eq+PartialEq+Default,V:Default{

    fn link_cl<'a,Key,Val>(&self, base_ptr:*mut Entity<Key,Val>, val:super::entity::EntityGuard<'a,Key,Val>, link:bool)
        where 
            Key:Hash+Ord+PartialOrd+Eq+PartialEq {
                
                let prev = unsafe {
                    base_ptr.add(
                        match link{
                            true=>val.link.prev as usize,
                            false=>val.chain.prev as usize
                        }
                    ).as_mut().unwrap().lock()
                };
                let next = unsafe {
                    base_ptr.add(
                        match link{
                            true=>val.link.next as usize,
                            false=>val.chain.next as usize
                        }
                    ).as_mut().unwrap().lock()
                };
                todo!()
    }
}