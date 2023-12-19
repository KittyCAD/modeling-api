use kittycad_modeling_cmds::shared::Angle;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ExecutionError;

/// A value stored in KCEP program memory.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Primitive {
    String(String),
    NumericValue(NumericPrimitive),
    Uuid(Uuid),
    Bytes(Vec<u8>),
    Bool(bool),
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

/// Angle is always stored as f64 degrees.
impl From<Angle> for Primitive {
    fn from(value: Angle) -> Self {
        Self::NumericValue(NumericPrimitive::Float(value.to_degrees()))
    }
}

impl From<Vec<u8>> for Primitive {
    fn from(value: Vec<u8>) -> Self {
        Self::Bytes(value)
    }
}

impl TryFrom<Primitive> for String {
    type Error = ExecutionError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        if let Primitive::String(s) = value {
            Ok(s)
        } else {
            Err(ExecutionError::MemoryWrongType {
                expected: "string",
                actual: format!("{value:?}"),
            })
        }
    }
}

impl TryFrom<Primitive> for Uuid {
    type Error = ExecutionError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        if let Primitive::Uuid(u) = value {
            Ok(u)
        } else {
            Err(ExecutionError::MemoryWrongType {
                expected: "uuid",
                actual: format!("{value:?}"),
            })
        }
    }
}

/// Angle is always stored as f64 degrees.
impl TryFrom<Primitive> for Angle {
    type Error = ExecutionError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        if let Primitive::NumericValue(x) = value {
            Ok(Angle::from_degrees(x.into()))
        } else {
            Err(ExecutionError::MemoryWrongType {
                expected: "number",
                actual: format!("{value:?}"),
            })
        }
    }
}

impl TryFrom<Primitive> for f64 {
    type Error = ExecutionError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        if let Primitive::NumericValue(NumericPrimitive::Float(x)) = value {
            Ok(x)
        } else {
            Err(ExecutionError::MemoryWrongType {
                expected: "float",
                actual: format!("{value:?}"),
            })
        }
    }
}

impl TryFrom<Primitive> for f32 {
    type Error = ExecutionError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        f64::try_from(value).map(|x| x as f32)
    }
}

impl TryFrom<Primitive> for Vec<u8> {
    type Error = ExecutionError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        if let Primitive::Bytes(x) = value {
            Ok(x)
        } else {
            Err(ExecutionError::MemoryWrongType {
                expected: "bytes",
                actual: format!("{value:?}"),
            })
        }
    }
}

impl TryFrom<Primitive> for bool {
    type Error = ExecutionError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        if let Primitive::Bool(x) = value {
            Ok(x)
        } else {
            Err(ExecutionError::MemoryWrongType {
                expected: "bool",
                actual: format!("{value:?}"),
            })
        }
    }
}

impl TryFrom<Primitive> for usize {
    type Error = ExecutionError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        if let Primitive::NumericValue(NumericPrimitive::Integer(x)) = value {
            Ok(x)
        } else {
            Err(ExecutionError::MemoryWrongType {
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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum NumericPrimitive {
    Integer(usize),
    Float(f64),
}

impl crate::value::Value for Primitive {
    fn into_parts(self) -> Vec<Primitive> {
        vec![self]
    }

    fn from_parts<I>(values: &mut I) -> Result<Self, ExecutionError>
    where
        I: Iterator<Item = Option<Primitive>>,
    {
        values
            .next()
            .and_then(|v| v.to_owned())
            .ok_or(ExecutionError::MemoryWrongSize)
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
