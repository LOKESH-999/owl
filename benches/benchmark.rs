#![feature(test)]  // Enables Rust's built-in benchmarking
extern crate test; // Import the test crate
use owl::core_owl::node::empty_line::EmptyMap;

use test::Bencher;


#[bench]
fn bench_get_empty_idx(b: &mut Bencher) {
    let mut map = EmptyMap::new();
    b.iter(|| {
            test::black_box(map.get_empty_idx());
    });
}

#[bench]
fn bench_return_free_idx(b: &mut Bencher) {
    let mut map = EmptyMap::new();
    let idx = map.get_empty_idx();
    b.iter(|| {
            test::black_box(map.return_free_idx(idx));
    });
}
