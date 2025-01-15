#![feature(portable_simd)]
#![feature(ptr_as_ref_unchecked)]

use owl::node::Node;


mod owl;
// mod node_components;
use owl::node_components::Entity;
fn main() {
    println!("Hello, world!{}",std::mem::size_of::<Entity<i32,i32>>());
}
