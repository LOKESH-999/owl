/// Represents an entity within the consistency ring, 
/// tracking the number of hashes pointing to a specific area (concentration).
#[derive(Clone,Copy)]
pub struct RingEntity {
    /// The number of hashes pointing to this specific area.
    concentration: u32,
}

impl RingEntity {
    /// Creates a new instance of `RingEntity` with the concentration value initialized to zero.
    /// 
    /// # Returns
    /// A new `RingEntity` instance with the concentration set to `0`.
    #[inline]
    pub const fn new() -> Self {
        RingEntity {
            concentration: 0,
        }
    }

    /// Increments the concentration value by 1, 
    /// signifying an additional hash now points/hits to this area.
    #[inline]
    pub fn inc(&mut self) {
        self.concentration += 1;
    }

    /// Returns the current concentration value.
    ///
    /// # Returns
    /// The number of hashes currently pointing to this area.
    #[inline]
    pub fn get(&self) -> u32 {
        self.concentration
    }

    /// Sets the concentration value to the specified number.
    ///
    /// # Arguments
    /// * `val` - The new concentration value indicating the number of hashes for this area.
    #[inline]
    pub fn set(&mut self, val: u32) {
        self.concentration = val;
    }
}
