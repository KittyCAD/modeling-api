use kittycad_execution_plan_traits::{MemoryError, Primitive, Value};

use crate::Address;

/// Helper wrapper around Memory. It lets you push static data into memory before the program runs.
pub struct StaticMemoryInitializer {
    memory: Memory,
    last: Address,
}

impl Default for StaticMemoryInitializer {
    fn default() -> Self {
        Self {
            memory: Default::default(),
            last: Address(0),
        }
    }
}

impl StaticMemoryInitializer {
    /// Finish putting static data into memory, get ready to execute the plan.
    /// Returns normal execution plan program memory.
    pub fn finish(self) -> Memory {
        self.memory
    }

    /// Put the next value into memory.
    /// Returns the address that the value was inserted at.
    pub fn push<T: Value>(&mut self, val: T) -> Address {
        let addr_of_value = self.last;
        let len = self.memory.set_composite(self.last, val);
        self.last = self.last.offset(len);
        addr_of_value
    }
}

/// KCEP's program memory. A flat, linear list of values.
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Memory {
    addresses: Vec<Option<Primitive>>,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            addresses: vec![None; 1024],
        }
    }
}

impl Memory {
    /// Get a value from KCEP's program memory.
    pub fn get(&self, Address(addr): &Address) -> Option<&Primitive> {
        self.addresses[*addr].as_ref()
    }

    /// Store a value in KCEP's program memory.
    pub fn set(&mut self, Address(addr): Address, value: Primitive) {
        // If isn't big enough for this value, double the size of memory until it is.
        while addr > self.addresses.len() {
            self.addresses.extend(vec![None; self.addresses.len()]);
        }
        self.addresses[addr] = Some(value);
    }

    /// Store a value value (i.e. a value which takes up multiple addresses in memory).
    /// Store its parts in consecutive memory addresses starting at `start`.
    /// Returns how many memory addresses the data took up.
    pub fn set_composite<T: Value>(&mut self, start: Address, composite_value: T) -> usize {
        let parts = composite_value.into_parts().into_iter();
        let mut total_addrs = 0;
        for (value, addr) in parts.zip(start.0..) {
            self.addresses[addr] = Some(value);
            total_addrs += 1;
        }
        total_addrs
    }

    /// Get a value value (i.e. a value which takes up multiple addresses in memory).
    /// Its parts are stored in consecutive memory addresses starting at `start`.
    pub fn get_composite<T: Value>(&self, start: Address) -> std::result::Result<T, MemoryError> {
        let mut values = self.addresses.iter().skip(start.0).cloned();
        T::from_parts(&mut values)
    }

    /// Iterate over each memory address and its value.
    pub fn iter(&self) -> impl Iterator<Item = (usize, &Option<Primitive>)> {
        self.addresses.iter().enumerate()
    }
}
