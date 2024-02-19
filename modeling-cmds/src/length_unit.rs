//! A length unit is a type that converts a length value from one unit to another.
//! In the case of the engine we will always convert to millimeters, since that is what the engine uses.
use kittycad_execution_plan_macros::ExecutionPlanValue;
use serde::{Deserialize, Serialize};

/// A length unit is wrapper around an f64 that represents a length in some unit.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, ExecutionPlanValue)]
pub struct LengthUnit(pub f64);

impl LengthUnit {
    /// Get the value of the length unit in millimeters.
    /// This is how we convert for the engine.
    pub fn to_millimeters(&self, from: crate::units::UnitLength) -> f64 {
        from.convert_to(crate::units::UnitLength::Millimeters, self.0)
    }

    /// Get the value from millimeters to the length unit.
    pub fn from_millimeters(&self, to: crate::units::UnitLength) -> f64 {
        crate::units::UnitLength::Millimeters.convert_to(to, self.0)
    }
}

impl schemars::JsonSchema for LengthUnit {
    fn schema_name() -> String {
        "LengthUnit".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <f64>::json_schema(gen)
    }
}

impl From<f64> for LengthUnit {
    fn from(value: f64) -> Self {
        LengthUnit(value)
    }
}

impl std::ops::Neg for LengthUnit {
    type Output = Self;

    fn neg(self) -> Self::Output {
        LengthUnit(-self.0)
    }
}

impl std::ops::Add for LengthUnit {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        LengthUnit(self.0 + rhs.0)
    }
}

impl std::ops::Sub for LengthUnit {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        LengthUnit(self.0 - rhs.0)
    }
}

impl std::ops::Mul<f64> for LengthUnit {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        LengthUnit(self.0 * rhs)
    }
}

impl std::ops::Div<f64> for LengthUnit {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        LengthUnit(self.0 / rhs)
    }
}
