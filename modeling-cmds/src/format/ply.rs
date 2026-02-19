use bon::Builder;
use parse_display::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{coord, format::Selection, units::UnitLength};

/// Import models in PLY format.
pub mod import {
    use super::*;

    /// Options for importing PLY.
    #[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Builder)]
    #[serde(rename = "PlyImportOptions")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    #[cfg_attr(
        feature = "python",
        pyo3_stub_gen::derive::gen_stub_pyclass,
        pyo3::pyclass(name = "PlyImportOptions")
    )]
    #[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
    pub struct Options {
        /// Co-ordinate system of input data.
        ///
        /// Defaults to the [KittyCAD co-ordinate system].
        ///
        /// [KittyCAD co-ordinate system]: ../coord/constant.KITTYCAD.html
        #[builder(default = *coord::KITTYCAD)]
        pub coords: coord::System,

        /// The units of the input data.
        ///
        /// This is very important for correct scaling and when calculating physics properties like
        /// mass, etc.
        ///
        /// Defaults to millimeters.
        #[builder(default = UnitLength::Millimeters)]
        pub units: UnitLength,
    }

    #[cfg(feature = "python")]
    #[pyo3_stub_gen::derive::gen_stub_pymethods]
    #[pyo3::pymethods]
    impl Options {
        #[new]
        /// Set the options to their defaults.
        pub fn new() -> Self {
            Default::default()
        }
    }

    impl Default for Options {
        fn default() -> Self {
            Self {
                coords: *coord::KITTYCAD,
                units: UnitLength::Millimeters,
            }
        }
    }
}

/// Export models in PLY format.
pub mod export {

    use super::*;

    /// Options for exporting PLY.
    #[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Builder)]
    #[serde(rename = "PlyExportOptions")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
    #[cfg_attr(
        feature = "python",
        pyo3_stub_gen::derive::gen_stub_pyclass,
        pyo3::pyclass(name = "PlyExportOptions")
    )]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    #[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
    pub struct Options {
        /// Co-ordinate system of output data.
        ///
        /// Defaults to the [KittyCAD co-ordinate system].
        ///
        /// [KittyCAD co-ordinate system]: ../coord/constant.KITTYCAD.html
        #[builder(default = *coord::KITTYCAD)]
        pub coords: coord::System,

        /// Export selection.
        #[builder(default)]
        pub selection: Selection,

        /// The storage for the output PLY file.
        #[builder(default)]
        pub storage: Storage,

        /// Export length unit.
        ///
        /// Defaults to millimeters.
        #[builder(default = UnitLength::Millimeters)]
        pub units: UnitLength,
    }

    #[cfg(feature = "python")]
    #[pyo3_stub_gen::derive::gen_stub_pymethods]
    #[pyo3::pymethods]
    impl Options {
        #[new]
        /// Set the options to their defaults.
        pub fn new() -> Self {
            Default::default()
        }
    }

    impl Default for Options {
        fn default() -> Self {
            Self {
                coords: *coord::KITTYCAD,
                selection: Default::default(),
                storage: Default::default(),
                units: UnitLength::Millimeters,
            }
        }
    }

    /// The storage for the output PLY file.
    #[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr, Default)]
    #[display(style = "snake_case")]
    #[serde(rename = "PlyStorage", rename_all = "snake_case")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    #[cfg_attr(
        feature = "python",
        pyo3_stub_gen::derive::gen_stub_pyclass_enum,
        pyo3::pyclass(name = "PlyStorage")
    )]
    #[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
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
