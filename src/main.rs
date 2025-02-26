#![feature(portable_simd)]
#![feature(ptr_as_ref_unchecked)]

// use owl::node::Node;
mod core_owl;

mod owl;
// mod node_components;
use owl::node_components::Entity;
fn main() {
    let r= vec![123];
    println!("Hello, world!{}",std::mem::size_of::<Entity<i32,i32>>());
    println!("{:?}",core_owl::owl_ring::RING.len());
}
