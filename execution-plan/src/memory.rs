use kittycad_execution_plan_traits::{
    InMemory, ListHeader, MemoryError, NumericPrimitive, ObjectHeader, Primitive, ReadMemory, Value,
};

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
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Memory {
    /// Each address of memory.
    pub addresses: Vec<Option<Primitive>>,
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
    /// Get a value from KCEP's program memory.
    fn get(&self, address: &Address) -> Option<&Primitive> {
        self.addresses[inner(*address)].as_ref()
    }

    /// Get a value value (i.e. a value which takes up multiple addresses in memory).
    /// Its parts are stored in consecutive memory addresses starting at `start`.
    fn get_composite<T: Value>(&self, start: Address) -> Result<(T, usize), MemoryError> {
        let mut values = self.addresses.iter().skip(inner(start)).cloned();
        T::from_parts(&mut values)
    }

    fn stack_pop(&mut self) -> Result<Vec<Primitive>, MemoryError> {
        self.stack.pop()
    }

    fn stack_peek(&self) -> Result<&Vec<Primitive>, MemoryError> {
        self.stack.peek()
    }
}

impl Memory {
    /// Store a value in KCEP's program memory.
    pub fn set(&mut self, address: Address, value: Primitive) {
        let addr = address - Address::ZERO;
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
        for (value, addr) in parts.zip(start - Address::ZERO..) {
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

    /// Look for either a usize or an object/list header at the given address.
    /// Return that usize, or the `size` field of the header.
    pub fn get_size(&self, addr: &Address) -> Result<usize, ExecutionError> {
        let primitive = self.get(addr).ok_or(ExecutionError::MemoryEmpty { addr: *addr })?;
        let size = match primitive {
            Primitive::NumericValue(NumericPrimitive::UInteger(size)) => *size,
            Primitive::ListHeader(ListHeader { count: _, size }) => *size,
            Primitive::ObjectHeader(ObjectHeader { properties: _, size }) => *size,
            other => {
                return Err(ExecutionError::MemoryError(MemoryError::MemoryWrongType {
                    expected: "ObjectHeader, ListHeader, or usize",
                    actual: format!("{other:?}"),
                }))
            }
        };
        Ok(size)
    }

    /// Get a range of addresses, starting at `start` and continuing for `len` more.
    pub fn get_slice(&self, start: Address, len: usize) -> Result<Vec<Primitive>, ExecutionError> {
        let slice = &self.addresses[inner(start)..inner(start) + len];
        let x = slice
            .iter()
            .cloned()
            .enumerate()
            .map(|(addr, prim)| {
                prim.ok_or(ExecutionError::MemoryEmpty {
                    addr: Address::ZERO + addr,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(x)
    }

    /// Read a T value out of memory (either addressable or stack).
    pub fn get_in_memory<T: Value>(&mut self, source: InMemory) -> Result<(T, usize), MemoryError> {
        match source {
            InMemory::Address(a) => self.get_composite(a),
            InMemory::StackPop => {
                let data = self.stack_pop()?;
                let mut data_parts = data.iter().cloned().map(Some);
                T::from_parts(&mut data_parts)
            }
            InMemory::StackPeek => {
                let data = self.stack_peek()?;
                let mut data_parts = data.iter().cloned().map(Some);
                T::from_parts(&mut data_parts)
            }
        }
    }

    /// Return a nicely-formatted table of stack.
    #[must_use]
    pub fn debug_table_stack(&self) -> String {
        #[derive(tabled::Tabled)]
        struct StackLevel {
            depth: usize,
            value: String,
        }

        let table_data: Vec<_> = self
            .stack
            .inner
            .iter()
            .enumerate()
            .map(|(depth, slice)| StackLevel {
                depth,
                value: format!("{slice:?}"),
            })
            .collect();
        tabled::Table::new(table_data)
            .with(tabled::settings::Style::sharp())
            .to_string()
    }

    /// Return a nicely-formatted table of memory.
    #[must_use]
    pub fn debug_table(&self, up_to: Option<usize>) -> String {
        #[derive(tabled::Tabled)]
        struct MemoryAddr {
            addr: String,
            val_type: &'static str,
            value: String,
        }
        let table_data: Vec<_> = self
            .iter()
            .filter_map(|(i, val)| {
                if let Some(val) = val {
                    let (val_type, value) = pretty_print(val);
                    Some(MemoryAddr {
                        addr: i.to_string(),
                        val_type,
                        value,
                    })
                } else if let Some(up_to) = up_to {
                    if i <= up_to {
                        Some(MemoryAddr {
                            addr: i.to_string(),
                            val_type: "",
                            value: "".to_owned(),
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        tabled::Table::new(table_data)
            .with(tabled::settings::Style::sharp())
            .to_string()
    }

    /// Get the address of the last non-empty address.
    /// If none, then all addresses are empty.
    #[must_use]
    pub fn last_nonempty_address(&self) -> Option<usize> {
        self.iter()
            .filter_map(|(i, v)| if v.is_some() { Some(i) } else { None })
            .last()
    }
}

fn pretty_print(p: &Primitive) -> (&'static str, String) {
    match p {
        Primitive::String(v) => ("String", v.to_owned()),
        Primitive::NumericValue(NumericPrimitive::Float(v)) => ("Float", v.to_string()),
        Primitive::NumericValue(NumericPrimitive::UInteger(v)) => ("Uint", v.to_string()),
        Primitive::NumericValue(NumericPrimitive::Integer(v)) => ("Int", v.to_string()),
        Primitive::Uuid(v) => ("Uuid", v.to_string()),
        Primitive::Bytes(_) => ("Bytes", String::new()),
        Primitive::Bool(v) => ("Bool", v.to_string()),
        Primitive::ListHeader(ListHeader { count, size }) => {
            ("List header", format!("{count} elements, {size} primitives"))
        }
        Primitive::ObjectHeader(ObjectHeader { properties, size }) => (
            "Object header",
            format!("keys {}, {size} primitives", properties.clone().join(",")),
        ),
        Primitive::Nil => ("Nil", String::new()),
        Primitive::Address(a) => ("Address", a.to_string()),
    }
}

fn inner(a: Address) -> usize {
    a - Address::ZERO
}

/// A stack where values can be pushed/popped.
#[derive(Debug, Eq, PartialEq, Default, Clone)]
pub struct Stack<T> {
    inner: Vec<T>,
}

impl<T> Stack<T> {
    /// Add a value to the top of the stack (above any previous values).
    pub fn push(&mut self, t: T) {
        self.inner.push(t);
    }
    /// Remove a value from the top of the stack, and return it.
    pub fn pop(&mut self) -> Result<T, MemoryError> {
        self.inner.pop().ok_or(MemoryError::StackEmpty)
    }
    /// Return the value from the top of the stack.
    pub fn peek(&self) -> Result<&T, MemoryError> {
        self.inner.last().ok_or(MemoryError::StackEmpty)
    }
    /// Is the stack empty?
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    /// Iterate over the stack, from top to bottom.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter().rev()
    }
    /// How many items are currently in the stack?
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl Stack<Vec<Primitive>> {
    /// Remove a value from the top of the stack, and return it.
    /// If it's a single primitive long, return Ok, otherwise error.
    pub fn pop_single(&mut self) -> Result<Primitive, ExecutionError> {
        let mut slice = self.pop()?;
        let prim = slice.pop().ok_or(MemoryError::StackNotPrimitive { actual_length: 0 })?;
        if !slice.is_empty() {
            return Err(ExecutionError::MemoryError(MemoryError::StackNotPrimitive {
                actual_length: slice.len() + 1,
            }));
        }
        Ok(prim)
    }
}
