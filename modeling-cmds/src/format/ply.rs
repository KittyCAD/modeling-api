use parse_display::{Display, FromStr};
use serde::{Deserialize, Serialize};

use crate::{coord, format::Selection, units::UnitLength};

/// Import models in PLY format.
pub mod import {
    use super::*;

    /// Options for importing PLY.
    #[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Display, FromStr)]
    #[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
    #[display("coords: {coords}, units: {units}")]
    #[serde(rename = "PlyImportOptions")]
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
                // We can default to meters and fix it later if needed.
                units: crate::units::UnitLength::Meters,
            }
        }
    }
}

/// Export models in PLY format.
pub mod export {

    use super::*;

    /// Options for exporting PLY.
    #[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Display, FromStr)]
    #[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
    #[display("coords: {coords}, selection: {selection}, storage: {storage}, units: {units}")]
    #[serde(rename = "PlyExportOptions")]
    pub struct Options {
        /// Co-ordinate system of output data.
        ///
        /// Defaults to the [KittyCAD co-ordinate system].
        ///
        /// [KittyCAD co-ordinate system]: ../coord/constant.KITTYCAD.html
        pub coords: coord::System,

        /// Export selection.
        pub selection: Selection,

        /// The storage for the output PLY file.
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

    /// The storage for the output PLY file.
    #[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Display, FromStr, Default)]
    #[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
    #[display(style = "snake_case")]
    #[serde(rename = "PlyStorage", rename_all = "snake_case")]
    pub enum Storage {
        /// Write numbers in their ascii representation (e.g. -13, 6.28, etc.). Properties are separated by spaces and elements are separated by line breaks.
        #[default]
        Ascii,
        /// Encode payload as binary using little endian.
        BinaryLittleEndian,
        /// Encode payload as binary using big endian.
        BinaryBigEndian,
    }
}
