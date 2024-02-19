use kittycad_execution_plan_traits::{impl_value_on_primitive_ish, MemoryError, NumericPrimitive, Primitive, Value};

use crate::{base64::Base64Data, id::ModelingCmdId, length_unit::LengthUnit, shared::Angle};

/// Angle is always stored as f64 degrees.
impl From<Angle> for Primitive {
    fn from(value: Angle) -> Self {
        Self::NumericValue(NumericPrimitive::Float(value.to_degrees()))
    }
}

/// Angle is always stored as f64 degrees.
impl TryFrom<Primitive> for Angle {
    type Error = MemoryError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        if let Primitive::NumericValue(x) = value {
            Ok(Angle::from_degrees(x.into()))
        } else {
            Err(MemoryError::MemoryWrongType {
                expected: "number",
                actual: format!("{value:?}"),
            })
        }
    }
}

impl From<LengthUnit> for Primitive {
    fn from(value: LengthUnit) -> Self {
        Self::NumericValue(NumericPrimitive::Float(value.0))
    }
}

impl TryFrom<Primitive> for LengthUnit {
    type Error = MemoryError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        if let Primitive::NumericValue(x) = value {
            Ok(LengthUnit(x.into()))
        } else {
            Err(MemoryError::MemoryWrongType {
                expected: "number",
                actual: format!("{value:?}"),
            })
        }
    }
}

impl From<Base64Data> for Primitive {
    fn from(value: Base64Data) -> Self {
        Self::Bytes(value.into())
    }
}

impl TryFrom<Primitive> for Base64Data {
    type Error = MemoryError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        Vec::<u8>::try_from(value).map(Base64Data::from)
    }
}

impl_value_on_primitive_ish!(Value, Angle);
impl_value_on_primitive_ish!(Value, Base64Data);
impl_value_on_primitive_ish!(Value, ModelingCmdId);
