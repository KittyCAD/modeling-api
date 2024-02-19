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
        from.as_measurement(self.0).as_millimeters()
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
