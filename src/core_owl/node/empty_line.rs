use super::array::ARR_SIZE;
use super::NULL_IDX;


const ELEMINATE_POINT:u64 = 0b1111111111111111111111111111111111111111111111111000000000000000;
const L2_LENGTH:u16 = 16;
const L3_LENGTH:u16 = 1024;


pub struct EmptyMap{
    pub l1: u16,
    pub count: u16,
    pub l2: [u64;16],
    pub l3: [u64;1024],
}


impl EmptyMap {
    pub const fn new() ->Self{
        let mut map = EmptyMap{
            l1:u16::MAX,
            count:0,
            l2:[u64::MAX;16],
            l3:[u64::MAX;1024]
        };
        map.l3[1023] = ELEMINATE_POINT;
        map
    }

    pub const fn get_empty_count(&self)->u16{
        ARR_SIZE - self.count
    }

    pub fn get_empty_idx(&mut self)->u16{
        if self.count == ARR_SIZE{
            return NULL_IDX;
        }
        let l1_trailing_zeros = self.l1.trailing_zeros() as u16;
        let l2_idx = 15 - l1_trailing_zeros;
        let l2_trailing_zeros = self.l2[l2_idx as usize].trailing_zeros() as u16;
        let l3_idx = ((63 - l2_trailing_zeros) + (l2_idx * 64)) ;
        let l3_trailing_zeros = self.l3[l3_idx as usize].trailing_zeros() as u16;
        // println!("{l1_trailing_zeros},{l2_idx},{l2_trailing_zeros},{l3_idx},{l3_trailing_zeros}");
        self.l3[l3_idx as usize] &= !(1 << l3_trailing_zeros as u64);
        match self.l3[l3_idx as usize] {
            0 => {
                self.l2[l2_idx as usize] &= !(1 << l2_trailing_zeros as u64);
                match self.l2[l2_idx as usize] {
                    0 => self.l1 &= !(1 << l1_trailing_zeros),
                    _=>{}
                }
            },
            _=>{}
        }
        self.count += 1;
        return l3_idx * 64 + (63 - l3_trailing_zeros);
    }

    pub fn return_idx(&mut self,idx : u16){
        todo!()
    }
}

