use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{impl_value_on_primitive_ish, MemoryError, Value};

/// A value stored in KCEP program memory.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Primitive {
    /// UTF-8 text
    String(String),
    /// Various number kinds
    NumericValue(NumericPrimitive),
    /// UUID
    Uuid(Uuid),
    /// Raw binary
    Bytes(Vec<u8>),
    /// True or false
    Bool(bool),
    /// An optional value which was not given.
    Nil,
}

impl From<bool> for Primitive {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<Uuid> for Primitive {
    fn from(u: Uuid) -> Self {
        Self::Uuid(u)
    }
}

impl From<String> for Primitive {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<f32> for Primitive {
    fn from(value: f32) -> Self {
        Self::NumericValue(NumericPrimitive::Float(value as f64))
    }
}

impl From<f64> for Primitive {
    fn from(value: f64) -> Self {
        Self::NumericValue(NumericPrimitive::Float(value))
    }
}

impl From<Vec<u8>> for Primitive {
    fn from(value: Vec<u8>) -> Self {
        Self::Bytes(value)
    }
}

impl TryFrom<Primitive> for String {
    type Error = MemoryError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        if let Primitive::String(s) = value {
            Ok(s)
        } else {
            Err(MemoryError::MemoryWrongType {
                expected: "string",
                actual: format!("{value:?}"),
            })
        }
    }
}

impl TryFrom<Primitive> for Uuid {
    type Error = MemoryError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        if let Primitive::Uuid(u) = value {
            Ok(u)
        } else {
            Err(MemoryError::MemoryWrongType {
                expected: "uuid",
                actual: format!("{value:?}"),
            })
        }
    }
}

impl TryFrom<Primitive> for f64 {
    type Error = MemoryError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        if let Primitive::NumericValue(NumericPrimitive::Float(x)) = value {
            Ok(x)
        } else {
            Err(MemoryError::MemoryWrongType {
                expected: "float",
                actual: format!("{value:?}"),
            })
        }
    }
}

impl TryFrom<Primitive> for f32 {
    type Error = MemoryError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        f64::try_from(value).map(|x| x as f32)
    }
}

impl TryFrom<Primitive> for Vec<u8> {
    type Error = MemoryError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        if let Primitive::Bytes(x) = value {
            Ok(x)
        } else {
            Err(MemoryError::MemoryWrongType {
                expected: "bytes",
                actual: format!("{value:?}"),
            })
        }
    }
}

impl TryFrom<Primitive> for bool {
    type Error = MemoryError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        if let Primitive::Bool(x) = value {
            Ok(x)
        } else {
            Err(MemoryError::MemoryWrongType {
                expected: "bool",
                actual: format!("{value:?}"),
            })
        }
    }
}

impl TryFrom<Primitive> for usize {
    type Error = MemoryError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        if let Primitive::NumericValue(NumericPrimitive::Integer(x)) = value {
            Ok(x)
        } else {
            Err(MemoryError::MemoryWrongType {
                expected: "usize",
                actual: format!("{value:?}"),
            })
        }
    }
}

impl From<usize> for Primitive {
    fn from(value: usize) -> Self {
        Self::NumericValue(NumericPrimitive::Integer(value))
    }
}

/// Various kinds of number.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum NumericPrimitive {
    /// Unsigned integer
    Integer(usize),
    /// Floating point
    Float(f64),
}

impl crate::Value for Primitive {
    fn into_parts(self) -> Vec<Primitive> {
        vec![self]
    }

    fn from_parts<I>(values: &mut I) -> Result<Self, MemoryError>
    where
        I: Iterator<Item = Option<Primitive>>,
    {
        values
            .next()
            .and_then(|v| v.to_owned())
            .ok_or(MemoryError::MemoryWrongSize)
    }
}

impl From<NumericPrimitive> for f64 {
    fn from(value: NumericPrimitive) -> Self {
        match value {
            NumericPrimitive::Integer(x) => x as f64,
            NumericPrimitive::Float(x) => x,
        }
    }
}

impl_value_on_primitive_ish!(Value, f32);
impl_value_on_primitive_ish!(Value, f64);
impl_value_on_primitive_ish!(Value, bool);
impl_value_on_primitive_ish!(Value, String);
impl_value_on_primitive_ish!(Value, Uuid);
type VecU8 = Vec<u8>;
impl_value_on_primitive_ish!(Value, VecU8);
impl_value_on_primitive_ish!(Value, usize);
