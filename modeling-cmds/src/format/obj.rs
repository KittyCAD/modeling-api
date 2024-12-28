
use parse_display::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{coord, units::UnitLength};

/// Import models in OBJ format.
pub mod import {
    use super::*;

    /// Options for importing OBJ.
    #[derive(
        Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr,
    )]
    #[display("coords: {coords}, units: {units}")]
    #[serde(rename = "ObjImportOptions")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    pub struct Options {
        /// Co-ordinate system of input data.
        ///
        /// Defaults to the [KittyCAD co-ordinate system].
        ///
        /// [KittyCAD co-ordinate system]: ../coord/constant.KITTYCAD.html
        pub coords: coord::System,
        /// The units of the input data.
        /// This is very important for correct scaling and when calculating physics properties like
        /// mass, etc.
        ///
        /// Defaults to meters.
        pub units: UnitLength,
    }

    impl Default for Options {
        fn default() -> Self {
            Self {
                coords: *coord::KITTYCAD,
                units: UnitLength::Meters,
            }
        }
    }
}

/// Export models in OBJ format.
pub mod export {
    use super::*;

    /// Options for exporting OBJ.
    #[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr)]
    #[display("coords: {coords}, units: {units}")]
    #[serde(rename = "ObjExportOptions")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    pub struct Options {
        /// Co-ordinate system of output data.
        ///
        /// Defaults to the [KittyCAD co-ordinate system].
        ///
        /// [KittyCAD co-ordinate system]: ../coord/constant.KITTYCAD.html
        pub coords: coord::System,

        /// Export length unit.
        ///
        /// Defaults to meters.
        pub units: UnitLength,
    }

    impl Default for Options {
        fn default() -> Self {
            Self {
                coords: *coord::KITTYCAD,
                units: UnitLength::Meters,
            }
        }
    }
}
