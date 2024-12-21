#![feature(portable_simd)]

use owl::node::Node;


mod owl;


fn main() {
    println!("Hello, world!{}",std::mem::size_of::<Node<&str,&str>>());
}
