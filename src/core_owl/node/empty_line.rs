use super::array::ARR_SIZE;
use super::NULL_IDX;


/// `EmptyMap` is a memory-efficient structure that efficiently handles the allocation
/// and deallocation of indices using bitwise operations. The structure tracks free and
/// occupied slots using a multi-level bitmask strategy that ensures quick access to available
/// slots for a fixed-size map. The map contains 3 layers (L1, L2, L3), where L1 is used to
/// track blocks, L2 for regions, and L3 for individual slots within the regions.
pub struct EmptyMap {
    /// The count of currently occupied indices.
    ///
    /// This field tracks the number of slots currently occupied (used). It is necessary
    /// for calculating the available (empty) slots.
    pub count: u16,

    /// The L1 filter is a 16-bit value tracking the availability of L2 blocks.
    ///
    /// A `1` in a position indicates the corresponding L2 block is available. A `0`
    /// indicates that the corresponding L2 block is occupied.
    pub l1_filter: u16,

    /// The L2 filter is an array of 16 `u64` values, each tracking the availability of
    /// sub-blocks within a specific region (L3 slots).
    ///
    /// Each `u64` bit represents the availability of one L3 slot.
    pub l2_filter: [u64; 16],

    /// The `free_slots` array tracks the availability of individual L3 slots.
    ///
    /// It contains 1024 `u64` values, where each bit represents an individual L3 slot.
    /// A `0` bit indicates a free slot, while a `1` bit indicates that slot is occupied.
    pub free_slots: [u64; 1024],
}

/// Constant values used to define the final L3 mask and the lengths of the L2 and L3 caches.
const FINAL_L3_MASK: u64 = 0b1111111111111111111111111111111111111111111111111000000000000000;
const L2_LENGTH: u16 = 16;  // Length of the L2 cache (number of L2 blocks).
const L3_LENGTH: u16 = 1024;  // Length of the L3 cache (number of L3 slots).

impl EmptyMap {
    /// Creates a new, empty `EmptyMap` instance.
    ///
    /// This constructor initializes the map with default values:
    /// - `l1_filter` is set to `u16::MAX` (all L2 blocks are available).
    /// - `count` is set to `0` (no slots are occupied).
    /// - `l2_filter` is set to `u64::MAX` for each L2 block (all L3 slots are available).
    /// - `free_slots` is set to `u64::MAX` for each L3 block (all slots are free).
    ///
    /// The final L3 block (index 1023) is set to `FINAL_L3_MASK` to manage the last slot.
    ///
    /// # Returns
    /// A new `EmptyMap` instance.
    pub const fn new() -> Self {
        let mut map = EmptyMap {
            l1_filter: u16::MAX,
            count: 0,
            l2_filter: [u64::MAX; 16],
            free_slots: [u64::MAX; 1024],
        };
        map.free_slots[1023] = FINAL_L3_MASK;  // Set the final L3 block mask.
        map
    }

    /// Returns the number of empty (free) slots in the map.
    ///
    /// The number of empty slots is calculated as the difference between the total size (`ARR_SIZE`)
    /// and the number of occupied slots (`count`).
    ///
    /// # Returns
    /// The number of empty slots as a `u16`.
    pub const fn get_empty_count(&self) -> u16 {
        ARR_SIZE - self.count
    }

    /// Finds and returns the index of the next available empty slot.
    ///
    /// The function uses a multi-level bitmask approach:
    /// 1. It checks the L1 filter to find the available L2 block.
    /// 2. It checks the L2 filter for the next available L3 slot.
    /// 3. The corresponding L3 slot is marked as occupied, and the filter is updated.
    /// 4. If a block becomes completely occupied, the function updates the L2 filter,
    ///    and if necessary, the L1 filter is also updated.
    ///
    /// If no free slots are available, it returns `NULL_IDX`.
    ///
    /// # Returns
    /// The index of the next available slot, or `NULL_IDX` if no slots are available.
    pub fn get_empty_idx(&mut self) -> u16 {
        // If all slots are full, return NULL_IDX.
        if self.count == ARR_SIZE {
            return NULL_IDX;
        }

        // Step 1: Check the L1 filter to identify the available L2 block.
        let l1_filter_trailing_zeros = self.l1_filter.trailing_zeros() as u16;
        let l2_filter_idx = 15 - l1_filter_trailing_zeros;

        // Step 2: Check the L2 filter to find an available L3 sub-slot.
        let l2_filter_trailing_zeros = self.l2_filter[l2_filter_idx as usize].trailing_zeros() as u16;
        let free_slots_idx = (63 - l2_filter_trailing_zeros) + (l2_filter_idx * 64);

        // Step 3: Check the specific L3 block and find the first available slot.
        let free_slots_trailing_zeros = self.free_slots[free_slots_idx as usize].trailing_zeros() as u16;

        let free_slot = &mut self.free_slots[free_slots_idx as usize];
        *free_slot &= !(1 << free_slots_trailing_zeros);  // Mark the slot as occupied.

        // Step 4: Adjust the L2 and L1 filters if necessary.
        if *free_slot == 0 {
            let l2_filter = &mut self.l2_filter[l2_filter_idx as usize];
            *l2_filter &= !(1 << l2_filter_trailing_zeros);

            if *l2_filter == 0 {
                self.l1_filter &= !(1 << l1_filter_trailing_zeros);
            }
        }

        self.count += 1;
        free_slots_idx * 64 + (63 - free_slots_trailing_zeros)  // Return the available index.
    }

    /// Returns an index to the pool of free slots.
    ///
    /// This function marks a given index as free again and updates the L1, L2, and L3
    /// filters accordingly:
    /// 1. The corresponding bit in the `free_slots` array is reset.
    /// 2. If an entire L3 block becomes free, the L2 filter is updated.
    /// 3. If an entire L2 block becomes free, the L1 filter is updated.
    ///
    /// # Arguments
    /// * `idx` - The index of the slot to be returned.
    pub fn return_free_idx(&mut self, idx: u16) {
        // Calculate which L3 block the given index belongs to. 
        let free_slots_idx = idx >> 6;  // Div by 64 (right shift by 6 bits)
    
        // Calculate the bit position of the index within the L3 block.
        let free_slots_bit_idx = 63 - (idx & 63);  // Reverse bit position in `u64`
    
        // Reference the free slot in the corresponding L3 block.
        let free_slot = &mut self.free_slots[free_slots_idx as usize];
        
        // Check if the current L3 block is entirely empty (i.e., has no free slots).
        if *free_slot == 0 {
            // If the L3 block is empty, update the L2 filter.
            let l2_filter_idx = (free_slots_idx >> 4) & 15;  // Calculate L2 block index (Div by 16 and mod by 16)
            let l2_filter_bit_idx = free_slots_idx & 63;  // Get position within the `l2_filter` array
    
            // If the L2 block is empty, update the L1 filter to indicate that a block in L2 is now available.
            if self.l2_filter[l2_filter_idx as usize] == 0 {
                self.l1_filter |= 1 << (15 - l2_filter_idx);  // Set the corresponding bit in the L1 filter.
            }
    
            // Mark the L2 block as having a free slot at the given index.
            self.l2_filter[l2_filter_idx as usize] |= 1 << l2_filter_bit_idx;
        }
        
        // Set the corresponding bit in the L3 block to indicate the slot is now free.
        *free_slot |= 1 << free_slots_bit_idx;
    
        // Decrease the count of allocated slots since one has been freed.
        self.count -= 1;
    }
}
