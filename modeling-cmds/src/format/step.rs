use bon::Builder;
use parse_display::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::coord;

/// Import models in STEP format.
pub mod import {
    use super::*;

    /// Options for importing STEP format.
    #[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr, Builder)]
    #[display("coords: {coords}, split_closed_faces: {split_closed_faces}")]
    #[serde(default, rename = "StepImportOptions")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    #[cfg_attr(
        feature = "python",
        pyo3_stub_gen::derive::gen_stub_pyclass,
        pyo3::pyclass(name = "StepImportOptions")
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

        /// Splits all closed faces into two open faces.
        ///
        /// Defaults to `false` but is implicitly `true` when importing into the engine.
        #[builder(default)]
        pub split_closed_faces: bool,
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
                split_closed_faces: false,
            }
        }
    }
}

/// Export models in STEP format.
pub mod export {
    use super::*;
    use crate::units::UnitLength;

    /// Describes the presentation style of the EXPRESS exchange format.
    #[derive(
        Default, Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr,
    )]
    #[display(style = "snake_case")]
    #[serde(rename = "StepPresentation", rename_all = "snake_case")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    #[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
    pub enum Presentation {
        /// Condenses the text to reduce the size of the file.
        Compact,

        /// Add extra spaces to make the text more easily readable.
        ///
        /// This is the default setting.
        #[default]
        Pretty,
    }

    /// Options for exporting STEP format.
    #[derive(Clone, Debug, Deserialize, Eq, Hash, JsonSchema, PartialEq, Serialize, Builder)]
    #[serde(rename = "StepExportOptions")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
    #[cfg_attr(
        feature = "python",
        pyo3_stub_gen::derive::gen_stub_pyclass,
        pyo3::pyclass(name = "StepExportOptions")
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

        /// Timestamp override.
        pub created: Option<chrono::DateTime<chrono::Utc>>,

        /// Export length unit.
        ///
        /// Defaults to meters.
        #[builder(default = UnitLength::Meters)]
        pub units: UnitLength,

        /// Presentation style.
        #[builder(default = Presentation::Pretty)]
        pub presentation: Presentation,
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
                created: None,
                units: UnitLength::Meters,
                presentation: Presentation::Pretty,
            }
        }
    }

    impl std::fmt::Display for Options {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "coords: {}, units: {}, presentation: {}",
                self.coords, self.units, self.presentation
            )
        }
    }

    impl std::str::FromStr for Options {
        type Err = <coord::System as std::str::FromStr>::Err;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self {
                coords: <coord::System as std::str::FromStr>::from_str(s)?,
                created: None,
                units: <UnitLength as std::str::FromStr>::from_str(s)?,
                presentation: <Presentation as std::str::FromStr>::from_str(s)?,
            })
        }
    }
}
