#![feature(portable_simd)]
#![feature(ptr_as_ref_unchecked)]


#![feature(test)]
extern crate test;


// use owl::node::Node;
pub mod core_owl;

mod owl;
// mod node_components;
use owl::node_components::Entity;
fn main() {
    let r= vec![123];
    println!("Hello, world!{}",std::mem::size_of::<Entity<i32,i32>>());
    println!("{:?}",core_owl::owl_ring::RING.len());
    let mut e_list = crate::core_owl::node::empty_line::EmptyMap::new();
    for _ in 0..65521{
        let len = e_list.get_empty_count();
        let idx = e_list.get_empty_idx();
        assert_eq!(len ,idx + 1);
        println!("{},{}",len,idx);
    }
}
