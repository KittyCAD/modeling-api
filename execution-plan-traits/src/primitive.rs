use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{impl_value_on_primitive_ish, Address, MemoryError, Value};

/// A value stored in KCEP program memory.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
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
    /// List metadata.
    ListHeader(ListHeader),
    /// Object metadata.
    ObjectHeader(ObjectHeader),
    /// An optional value which was not given.
    Nil,
    /// Address in KCEP memory.
    Address(Address),
}

impl std::fmt::Debug for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::String(s) => write!(f, r#""{s}""#),
            Primitive::NumericValue(NumericPrimitive::Float(x)) => x.fmt(f),
            Primitive::NumericValue(NumericPrimitive::Integer(x)) => x.fmt(f),
            Primitive::NumericValue(NumericPrimitive::UInteger(x)) => write!(f, "{x} (uint)"),
            Primitive::Uuid(u) => write!(f, "{u}"),
            Primitive::Bytes(_) => write!(f, "Binary"),
            Primitive::Bool(b) => write!(f, "{b}"),
            Primitive::ListHeader(ListHeader { count, size }) => write!(f, "List header (count {count}, size {size})"),
            Primitive::ObjectHeader(ObjectHeader { properties, size }) => {
                write!(f, "Object header (props {properties:?}, size {size})")
            }
            Primitive::Nil => write!(f, "Nil"),
            Primitive::Address(a) => write!(f, "Addr({})", a.0),
        }
    }
}

/// List metadata.
/// A KCEP list is a layout for variable-length lists whose elements may be different types,
/// and are identified by a uint index.
#[derive(Clone, Copy, Eq, PartialEq, Deserialize, Serialize, Debug)]
pub struct ListHeader {
    /// How many elements are in the list?
    pub count: usize,
    /// How many addresses does the list take up in total?
    pub size: usize,
}

/// Object metadata.
/// A KCEP object is a layout for objects whose elements may be different types,
/// and are identified by a string key.
#[derive(Clone, Eq, PartialEq, Deserialize, Serialize, Debug)]
pub struct ObjectHeader {
    /// What properties does the object have, and in what order are they laid out?
    pub properties: Vec<String>,
    /// What is the total size of this object
    pub size: usize,
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

impl From<ListHeader> for Primitive {
    fn from(value: ListHeader) -> Self {
        Self::ListHeader(value)
    }
}

impl From<ObjectHeader> for Primitive {
    fn from(value: ObjectHeader) -> Self {
        Self::ObjectHeader(value)
    }
}

impl From<Address> for Primitive {
    fn from(value: Address) -> Self {
        Self::Address(value)
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

impl TryFrom<Primitive> for Address {
    type Error = MemoryError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        if let Primitive::Address(x) = value {
            Ok(x)
        } else {
            Err(MemoryError::MemoryWrongType {
                expected: "address",
                actual: format!("{value:?}"),
            })
        }
    }
}

impl TryFrom<Primitive> for usize {
    type Error = MemoryError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        if let Primitive::NumericValue(NumericPrimitive::UInteger(x)) = value {
            Ok(x)
        } else {
            Err(MemoryError::MemoryWrongType {
                expected: "usize",
                actual: format!("{value:?}"),
            })
        }
    }
}

impl TryFrom<Primitive> for u32 {
    type Error = MemoryError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        if let Primitive::NumericValue(NumericPrimitive::UInteger(x)) = value {
            Ok(x.try_into().map_err(|_| MemoryError::MemoryWrongType {
                expected: "u32",
                actual: x.to_string(),
            })?)
        } else {
            Err(MemoryError::MemoryWrongType {
                expected: "u32",
                actual: format!("{value:?}"),
            })
        }
    }
}

impl TryFrom<Primitive> for ListHeader {
    type Error = MemoryError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        if let Primitive::ListHeader(x) = value {
            Ok(x)
        } else {
            Err(MemoryError::MemoryWrongType {
                expected: "ListHeader",
                actual: format!("{value:?}"),
            })
        }
    }
}

impl TryFrom<Primitive> for ObjectHeader {
    type Error = MemoryError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        if let Primitive::ObjectHeader(x) = value {
            Ok(x)
        } else {
            Err(MemoryError::MemoryWrongType {
                expected: "ObjectHeader",
                actual: format!("{value:?}"),
            })
        }
    }
}

impl TryFrom<Primitive> for i64 {
    type Error = MemoryError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        if let Primitive::NumericValue(NumericPrimitive::Integer(x)) = value {
            Ok(x)
        } else {
            Err(MemoryError::MemoryWrongType {
                expected: "i64",
                actual: format!("{value:?}"),
            })
        }
    }
}

impl From<usize> for Primitive {
    fn from(value: usize) -> Self {
        Self::NumericValue(NumericPrimitive::UInteger(value))
    }
}

impl From<u32> for Primitive {
    fn from(value: u32) -> Self {
        Self::NumericValue(NumericPrimitive::UInteger(value as usize))
    }
}

impl From<i64> for Primitive {
    fn from(value: i64) -> Self {
        Self::NumericValue(NumericPrimitive::Integer(value))
    }
}

/// Various kinds of number.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum NumericPrimitive {
    /// Unsigned integer
    UInteger(usize),
    /// Signed integer
    Integer(i64),
    /// Floating point
    Float(f64),
}

impl crate::Value for Primitive {
    fn into_parts(self) -> Vec<Primitive> {
        vec![self]
    }

    fn from_parts<I>(values: &mut I) -> Result<(Self, usize), MemoryError>
    where
        I: Iterator<Item = Option<Primitive>>,
    {
        values
            .next()
            .and_then(|v| v.to_owned())
            .ok_or(MemoryError::MemoryWrongSize)
            .map(|prim| (prim, 1))
    }
}

impl From<NumericPrimitive> for f64 {
    fn from(value: NumericPrimitive) -> Self {
        match value {
            NumericPrimitive::UInteger(x) => x as f64,
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
impl_value_on_primitive_ish!(Value, u32);
impl_value_on_primitive_ish!(Value, i64);
