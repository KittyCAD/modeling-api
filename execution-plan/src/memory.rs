use kittycad_execution_plan_traits::{MemoryError, Primitive, ReadMemory, Value};

use crate::{Address, ExecutionError};

/// Helper wrapper around Memory. It lets you push static data into memory before the program runs.
pub struct StaticMemoryInitializer {
    memory: Memory,
    last: Address,
}

impl Default for StaticMemoryInitializer {
    fn default() -> Self {
        Self {
            memory: Default::default(),
            last: Address::ZERO,
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
    /// A stack where temporary values can be pushed or popped.
    pub stack: Stack<Vec<Primitive>>,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            addresses: vec![None; 1024],
            stack: Stack::default(),
        }
    }
}

impl kittycad_execution_plan_traits::ReadMemory for Memory {
    type Address = crate::Address;

    /// Get a value from KCEP's program memory.
    fn get(&self, Address(addr): &Address) -> Option<&Primitive> {
        self.addresses[*addr].as_ref()
    }

    /// Get a value value (i.e. a value which takes up multiple addresses in memory).
    /// Its parts are stored in consecutive memory addresses starting at `start`.
    fn get_composite<T: Value>(&self, start: Address) -> std::result::Result<T, MemoryError> {
        let mut values = self.addresses.iter().skip(start.0).cloned();
        T::from_parts(&mut values)
    }
}

impl Memory {
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

    /// Iterate over each memory address and its value.
    pub fn iter(&self) -> impl Iterator<Item = (usize, &Option<Primitive>)> {
        self.addresses.iter().enumerate()
    }

    /// Get a primitive from `addr`. If it's of type T, extract that T. Otherwise error.
    pub fn get_primitive<T>(&self, addr: &Address) -> Result<T, ExecutionError>
    where
        T: TryFrom<Primitive, Error = MemoryError>,
    {
        let primitive = self
            .get(addr)
            .cloned()
            .ok_or(ExecutionError::MemoryEmpty { addr: *addr })?;
        primitive.try_into().map_err(ExecutionError::MemoryError)
    }

    /// Get a range of addresses, starting at `start` and continuing for `len` more.
    pub fn get_slice(&self, start: Address, len: usize) -> Result<Vec<Primitive>, ExecutionError> {
        let slice = &self.addresses[start.0..start.0 + len];
        let x = slice
            .iter()
            .cloned()
            .enumerate()
            .map(|(addr, prim)| prim.ok_or(ExecutionError::MemoryEmpty { addr: Address(addr) }))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(x)
    }
}

/// A stack where values can be pushed/popped.
#[derive(Debug, Eq, PartialEq, Default)]
pub struct Stack<T> {
    inner: Vec<T>,
}

impl<T> Stack<T> {
    /// Add a value to the top of the stack (above any previous values).
    pub fn push(&mut self, t: T) {
        self.inner.push(t);
    }
    /// Remove a value from the top of the stack, and return it.
    pub fn pop(&mut self) -> Result<T, ExecutionError> {
        self.inner.pop().ok_or(ExecutionError::StackEmpty)
    }
    /// Is the stack empty?
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl Stack<Vec<Primitive>> {
    pub fn pop_single(&mut self) -> Result<Primitive, ExecutionError> {
        let mut slice = self.pop()?;
        let prim = slice
            .pop()
            .ok_or(ExecutionError::StackNotPrimitive { actual_length: 0 })?;
        if !slice.is_empty() {
            return Err(ExecutionError::StackNotPrimitive {
                actual_length: slice.len() + 1,
            });
        }
        Ok(prim)
    }
}
