//! TODO
// pub use impl_value_on_primitive_ish;

pub use self::primitive::{NumericPrimitive, Primitive};

#[macro_use]
mod primitive;

/// Types that can be written to or read from KCEP program memory.
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

/// TODO
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
