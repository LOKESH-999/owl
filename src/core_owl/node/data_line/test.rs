use std::ptr::NonNull;

use super::entity::{Entity, Link};
use super::super::NULL_IDX;
use super::entity_iter::EntityIter;
#[test]
pub fn entity_iter_link_based1(){
    let mut arr = vec![];
    let mut prev = NULL_IDX;

    for i in 0..10000{
        let li =Link{
            next:i + 1,
            prev: prev
        };
        arr.push(Entity::new(i, i, li, Link::default()));
        prev = i;
    }
    arr[9999].link.next = NULL_IDX;
    let mut cur_idx = 2;
    for val in EntityIter::<u16,u16>::new(NonNull::new( arr.as_mut_ptr() ).unwrap(), cur_idx, false).unwrap().into_iter() {
        assert!(val.0.is_same_key(&cur_idx));
        assert!(val.1==cur_idx);
        cur_idx += 1
    }
}