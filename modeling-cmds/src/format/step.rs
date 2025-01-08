use parse_display::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::coord;

/// Import models in STEP format.
pub mod import {
    use super::*;

    /// Options for importing STEP format.
    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr)]
    #[display("split_closed_faces: {split_closed_faces}")]
    #[serde(default, rename = "StepImportOptions")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    pub struct Options {
        /// Splits all closed faces into two open faces.
        ///
        /// Defaults to `false` but is implicitly `true` when importing into the engine.
        pub split_closed_faces: bool,
    }
}

/// Export models in STEP format.
pub mod export {
    use super::*;

    /// Options for exporting STEP format.
    #[derive(Clone, Debug, Deserialize, Eq, Hash, JsonSchema, PartialEq, Serialize)]
    #[serde(rename = "StepExportOptions")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    pub struct Options {
        /// Co-ordinate system of output data.
        ///
        /// Defaults to the [KittyCAD co-ordinate system].
        ///
        /// [KittyCAD co-ordinate system]: ../coord/constant.KITTYCAD.html
        pub coords: coord::System,
        /// Timestamp override.
        ///
        /// This is intended for local integration testing only; it is not provided as an option
        /// in the JSON schema.
        #[serde(skip)]
        pub created: Option<chrono::DateTime<chrono::Utc>>,
    }

    impl Default for Options {
        fn default() -> Self {
            Self {
                coords: *coord::KITTYCAD,
                created: None,
            }
        }
    }

    impl std::fmt::Display for Options {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "coords: {}", self.coords)
        }
    }

    impl std::str::FromStr for Options {
        type Err = <coord::System as std::str::FromStr>::Err;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self {
                coords: <coord::System as std::str::FromStr>::from_str(s)?,
                created: None,
            })
        }
    }
}
