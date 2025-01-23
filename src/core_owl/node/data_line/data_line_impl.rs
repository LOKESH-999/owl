use std::hash::Hash;

use super::entity::{Entity, EntityGuard};
use super::entity_iter::EntityIter;

pub trait DataLineImpl<K,V> 
where K:Hash+Ord+PartialOrd+Eq+PartialEq{
    fn get_ref(&self,idx:u16)->&Entity<K,V>;

    fn get_mut(&mut self,idx:u16)->&mut Entity<K,V>;

    fn take(&self,key:&K,idx:u16)->Option<Entity<K,V>>;

    fn set_val(&mut self,val:V,idx:u16);

    fn as_ptr(&self)->*mut Entity<K,V>;
    
    fn lock_entity(&self,idx:u16) ->EntityGuard<'_,K,V>;

    fn entity_iter(&self,idx:u16,link:bool)->EntityIter<K,V>;
}

pub trait ChainLinker{
    fn link_cl<'a,K,V>(&self, base_ptr:*mut Entity<K,V>, val:EntityGuard<'a,K,V>, link:bool)
    where K:Hash+Ord+PartialOrd+Eq+PartialEq;
}