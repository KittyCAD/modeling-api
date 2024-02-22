//! Traits and types used for KittyCAD Execution Plans.

pub use self::address::Address;
pub use self::primitive::{ListHeader, NumericPrimitive, ObjectHeader, Primitive};

mod address;
mod containers;
#[macro_use]
mod primitive;

/// Types that can be written to or read from KCEP program memory, in one contiguous block.
/// If they require multiple memory addresses, they will be laid out
/// into multiple consecutive memory addresses.
pub trait Value: Sized {
    /// Store the value in memory.
    fn into_parts(self) -> Vec<Primitive>;
    /// Read the value from memory.
    fn from_parts<I>(values: &mut I) -> Result<Self, MemoryError>
    where
        I: Iterator<Item = Option<Primitive>>;
}

/// Types that can be read from KCEP program memory,
/// scattered across multiple places in the address space.
pub trait FromMemory: Sized {
    /// Read this type from memory, getting each field of the type from a different memory address.
    fn from_memory<I, M>(fields: &mut I, mem: &M) -> Result<Self, MemoryError>
    where
        M: ReadMemory,
        I: Iterator<Item = Address>;
}

/// Memory that a KittyCAD Execution Plan can read from.
pub trait ReadMemory {
    /// Get a value from the given address.
    fn get(&self, addr: &Address) -> Option<&Primitive>;
    /// Get a value from the given starting address. Value might require multiple addresses.
    fn get_composite<T: Value>(&self, start: Address) -> Result<T, MemoryError>;
    /// Remove the value on top of the stack, return it.
    fn stack_pop(&mut self) -> Result<Vec<Primitive>, MemoryError>;
    /// Return the value on top of the stack.
    fn stack_peek(&self) -> Result<&Vec<Primitive>, MemoryError>;
}

/// Errors that could occur when reading a type from KittyCAD Execution Plan program memory.
#[derive(Debug, thiserror::Error, Default)]
pub enum MemoryError {
    /// Something went wrong
    #[error("Something went wrong")]
    #[default]
    MemoryWrongSize,
    /// Type error, memory contained the wrong type.
    #[error("Tried to read a '{expected}' from KCEP program memory, found an '{actual}' instead")]
    MemoryWrongType {
        /// What the KittyCAD executor expected memory to contain
        expected: &'static str,
        /// What was actually in memory
        actual: String,
    },
    /// When trying to read an enum from memory, found a variant tag which is not valid for this enum.
    #[error("Found an unexpected tag '{actual}' when trying to read an enum of type {expected_type} from memory")]
    InvalidEnumVariant {
        /// What type of enum was being read from memory.
        expected_type: String,
        /// The actual enum tag found in memory.
        actual: String,
    },
    /// Stack is empty
    #[error("Stack is empty")]
    StackEmpty,
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

            fn from_parts<I>(values: &mut I) -> Result<Self, MemoryError>
            where
                I: Iterator<Item = Option<Primitive>>,
            {
                values
                    .next()
                    .ok_or(MemoryError::MemoryWrongSize)?
                    .to_owned()
                    .ok_or(MemoryError::MemoryWrongSize)?
                    .try_into()
            }
        }
    };
}
