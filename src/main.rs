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
}
