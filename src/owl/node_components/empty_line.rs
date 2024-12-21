
#![allow(unused)]

use crate::owl::array::Array;

/// Stores The Empty Idx of Entity
pub struct EmptyLine{
    arr:Array<u16>,
    len:usize,
}
impl EmptyLine {
    pub fn new(size:usize)->Self{
        EmptyLine{
            arr: Array::new(size),
            len:size 
        }
    }
    pub fn init(&mut self){
        for value in 0..self.len{
            unsafe {
                self.arr.set(value as u16, value);   
            }
        }
    }
}