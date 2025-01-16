use parse_display::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{coord, format::Selection, units::UnitLength};

/// Import models in STL format.
pub mod import {
    use super::*;

    /// Options for importing STL.
    #[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr)]
    #[display("coords: {coords}, units: {units}")]
    #[serde(rename = "StlImportOptions")]
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
        pub units: crate::units::UnitLength,
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

/// Export models in STL format.
pub mod export {

    use super::*;

    /// Options for exporting STL.
    #[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr)]
    #[display("coords: {coords}, selection: {selection}, storage: {storage}, units: {units}")]
    #[serde(rename = "StlExportOptions")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    pub struct Options {
        /// Co-ordinate system of output data.
        ///
        /// Defaults to the [KittyCAD co-ordinate system].
        ///
        /// [KittyCAD co-ordinate system]: ../coord/constant.KITTYCAD.html
        pub coords: coord::System,

        /// Export selection.
        pub selection: Selection,

        /// Export storage.
        pub storage: Storage,

        /// Export length unit.
        ///
        /// Defaults to meters.
        pub units: UnitLength,
    }

    impl Default for Options {
        fn default() -> Self {
            Self {
                coords: *coord::KITTYCAD,
                selection: Default::default(),
                storage: Default::default(),
                units: UnitLength::Meters,
            }
        }
    }

    /// Export storage.
    #[derive(
        Clone, Copy, Debug, Default, Deserialize, Display, Eq, FromStr, Hash, JsonSchema, PartialEq, Serialize,
    )]
    #[display(style = "snake_case")]
    #[serde(rename = "StlStorage", rename_all = "snake_case")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    pub enum Storage {
        /// Plaintext encoding.
        Ascii,

        /// Binary STL encoding.
        ///
        /// This is the default setting.
        #[default]
        Binary,
    }
}
