#![allow(dead_code)]
use std::hash::{Hash, Hasher};

use xxhash_rust::xxh3::xxh3_64_with_seed;

pub struct XXHasher{
    seed:u64,
    p_cache:u64
}

impl XXHasher {
    pub fn build(seed:u64)->Self{
        XXHasher{
            seed,
            p_cache:0
        }
    }
    pub fn hash<T:Hash>(&mut self,val:T)->u64{
        val.hash(self);
        self.p_cache
    }
}

impl Default for XXHasher {
    fn default() -> Self {
        XXHasher::build(102_73)
    }
}

impl Hasher for XXHasher {
    fn write(&mut self, bytes: &[u8]) {
        self.p_cache = xxh3_64_with_seed(bytes, self.seed)
    }
    fn finish(&self) -> u64 {
        self.p_cache
    }
}