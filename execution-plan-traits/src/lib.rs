//! Traits and types used for KittyCAD Execution Plans.

pub use self::address::Address;
use self::events::EventWriter;
pub use self::primitive::{ListHeader, NumericPrimitive, ObjectHeader, Primitive};

use serde::{Deserialize, Serialize};

mod address;
mod containers;
pub mod events;
#[macro_use]
mod primitive;

/// Types that can be written to or read from KCEP program memory, in one contiguous block.
/// If they require multiple memory addresses, they will be laid out
/// into multiple consecutive memory addresses.
pub trait Value: Sized {
    /// Store the value in memory.
    fn into_parts(self) -> Vec<Primitive>;
    /// Read the value from memory.
    fn from_parts<I>(values: &mut I) -> Result<(Self, usize), MemoryError>
    where
        I: Iterator<Item = Option<Primitive>>;
}

/// Types that can be read from KCEP program memory,
/// scattered across multiple places in the address space.
pub trait FromMemory: Sized {
    /// Read this type from memory, getting each field of the type from a different memory address.
    fn from_memory<I, M>(fields: &mut I, mem: &mut M, events: &mut EventWriter) -> Result<Self, MemoryError>
    where
        M: ReadMemory,
        I: Iterator<Item = InMemory>;
}

/// Where in memory a value is.
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum InMemory {
    /// At the given address.
    Address(Address),
    /// Top of stack. Pop the value after it's read.
    StackPop,
    /// Top of stack. Leave the value there after it's read.
    StackPeek,
}

impl From<Address> for InMemory {
    fn from(a: Address) -> Self {
        Self::Address(a)
    }
}

/// Select a memory area.
/// Intended to use for storing return values.
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum MemoryArea {
    /// At the given address.
    Address(Address),
    /// Push to stack.
    Stack,
}

/// Memory that a KittyCAD Execution Plan can read from.
pub trait ReadMemory {
    /// Get a value from the given address.
    fn get(&self, addr: &Address) -> Option<&Primitive>;
    /// Same as get but match the return signature of stack operations.
    fn get_ok(&self, addr: &Address) -> Result<Vec<Primitive>, MemoryError>;
    /// Get a value from the given starting address. Value might require multiple addresses.
    fn get_composite<T: Value>(&self, start: Address) -> Result<(T, usize), MemoryError>;
    /// Remove the value on top of the stack, return it.
    fn stack_pop(&mut self) -> Result<Vec<Primitive>, MemoryError>;
    /// Return the value on top of the stack.
    fn stack_peek(&self) -> Result<Vec<Primitive>, MemoryError>;
}

/// Errors that could occur when reading a type from KittyCAD Execution Plan program memory.
#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    /// Something went wrong
    #[error("Memory was wrong size")]
    MemoryWrongSize,
    /// Something went very wrong
    #[error("Bad memory access")]
    MemoryBadAccess,
    /// Type error, memory contained the wrong type.
    #[error("Tried to read a '{expected}' from KCEP program memory, found an '{actual}' instead")]
    MemoryWrongType {
        /// What the KittyCAD executor expected memory to contain
        expected: &'static str,
        /// What was actually in memory
        actual: String,
    },
    /// When trying to read an enum from memory, found a variant tag which is not valid for this enum.
    #[error("Found an unexpected tag '{actual}' when trying to read an enum of type {expected_type} from memory. Looking for one of {}", csv(.valid_variants))]
    InvalidEnumVariant {
        /// What type of enum was being read from memory.
        expected_type: String,
        /// The actual enum tag found in memory.
        actual: String,
        /// Which values would be acceptable?
        valid_variants: Vec<&'static str>,
    },
    /// Stack is empty
    #[error("Stack is empty")]
    StackEmpty,
    /// Stack should have contained a single primitive but it had a composite value instead.
    #[error("Expected stack to contain a single primitive, but it had a slice of length {actual_length}")]
    StackNotPrimitive {
        /// The actual size of the data that was popped off the stack
        /// Expected to be 1, but it was something else.
        actual_length: usize,
    },
}

fn csv(v: &[&'static str]) -> String {
    v.join(", ")
}

/// Macro to generate an `impl Value` for the given type `$subject`.
/// The type `$subject` must be "primitive-ish",
/// i.e. something that can be converted Into a Primitive and TryFrom a primitive
#[macro_export]
macro_rules! impl_value_on_primitive_ish {
    ($trait:ident, $subject:ident) => {
        impl $trait for $subject {
            fn into_parts(self) -> Vec<Primitive> {
                vec![self.into()]
            }

            fn from_parts<I>(values: &mut I) -> Result<(Self, usize), MemoryError>
            where
                I: Iterator<Item = Option<Primitive>>,
            {
                values
                    .next()
                    .ok_or(MemoryError::MemoryWrongSize)?
                    .to_owned()
                    .ok_or(MemoryError::MemoryWrongSize)?
                    .try_into()
                    .map(|prim| (prim, 1))
            }
        }
    };
}
