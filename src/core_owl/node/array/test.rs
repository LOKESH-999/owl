use crate::core_owl::node::array::ARR_SIZE;

use super::unsafe_array::UnsafeArray;

#[test]
pub fn simd_test(){
    let mut arr = UnsafeArray::simd_default(u16::MAX, ARR_SIZE as usize);
    let sl = unsafe { std::slice::from_raw_parts(arr.as_ptr(), ARR_SIZE as usize) };
    let right = [u16::MAX;ARR_SIZE as usize].as_slice();
    // assert!(sl.len()==10);
    assert_eq!(*sl,*right)
}