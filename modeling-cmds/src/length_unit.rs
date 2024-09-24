//! A length unit is a type that converts a length value from one unit to another.
//! In the case of the engine we will always convert to millimeters, since that is what the engine uses.

use serde::{Deserialize, Serialize};

use crate::shared::{Point2d, Point3d, Point4d};

/// A length unit is wrapper around an f64 that represents a length in some unit.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct LengthUnit(pub f64);

impl LengthUnit {
    /// Get the value of the length unit in millimeters.
    /// This is how we convert for the engine.
    pub fn to_millimeters(&self, from: crate::units::UnitLength) -> f64 {
        from.convert_to(crate::units::UnitLength::Millimeters, self.0)
    }

    /// Get the value from millimeters to the length unit.
    pub fn from_millimeters(&self, to: crate::units::UnitLength) -> LengthUnit {
        LengthUnit(crate::units::UnitLength::Millimeters.convert_to(to, self.0))
    }
}

impl Point3d<LengthUnit> {
    /// Convert the point to millimeters.
    pub fn to_millimeters(&self, from: crate::units::UnitLength) -> Point3d<f64> {
        Point3d {
            x: self.x.to_millimeters(from),
            y: self.y.to_millimeters(from),
            z: self.z.to_millimeters(from),
        }
    }
}

impl Point3d<f64> {
    /// Convert the point from millimeters.
    pub fn from_millimeters(&self, to: crate::units::UnitLength) -> Point3d<LengthUnit> {
        Point3d {
            x: crate::units::UnitLength::Millimeters.convert_to(to, self.x).into(),
            y: crate::units::UnitLength::Millimeters.convert_to(to, self.y).into(),
            z: crate::units::UnitLength::Millimeters.convert_to(to, self.z).into(),
        }
    }
}

impl Point2d<LengthUnit> {
    /// Convert the point to millimeters.
    pub fn to_millimeters(&self, from: crate::units::UnitLength) -> Point2d<f64> {
        Point2d {
            x: self.x.to_millimeters(from),
            y: self.y.to_millimeters(from),
        }
    }
}

impl Point2d<f64> {
    /// Convert the point from millimeters.
    pub fn from_millimeters(&self, to: crate::units::UnitLength) -> Point2d<LengthUnit> {
        Point2d {
            x: crate::units::UnitLength::Millimeters.convert_to(to, self.x).into(),
            y: crate::units::UnitLength::Millimeters.convert_to(to, self.y).into(),
        }
    }
}

impl Point4d<LengthUnit> {
    /// Convert the point to millimeters.
    pub fn to_millimeters(&self, from: crate::units::UnitLength) -> Point4d<f64> {
        Point4d {
            x: self.x.to_millimeters(from),
            y: self.y.to_millimeters(from),
            z: self.z.to_millimeters(from),
            w: self.w.0,
        }
    }
}

impl Point4d<f64> {
    /// Convert the point from millimeters.
    pub fn from_millimeters(&self, to: crate::units::UnitLength) -> Point4d<LengthUnit> {
        Point4d {
            x: crate::units::UnitLength::Millimeters.convert_to(to, self.x).into(),
            y: crate::units::UnitLength::Millimeters.convert_to(to, self.y).into(),
            z: crate::units::UnitLength::Millimeters.convert_to(to, self.z).into(),
            w: LengthUnit(self.w),
        }
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
