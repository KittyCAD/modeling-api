use parse_display::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Import models in FBX format.
pub mod import {
    use super::*;
    /// Options for importing FBX.
    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr)]
    #[display("")]
    #[serde(rename = "FbxImportOptions")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    pub struct Options {}
}

/// Export models in FBX format.
pub mod export {
    use super::*;

    /// Options for exporting FBX.
    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema)]
    #[serde(rename = "FbxExportOptions")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    pub struct Options {
        /// Specifies which kind of FBX will be exported.
        pub storage: Storage,

        /// Timestamp override.
        pub created: Option<chrono::DateTime<chrono::Utc>>,
    }

    impl std::fmt::Display for Options {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "storage: {}", self.storage)
        }
    }

    impl std::str::FromStr for Options {
        type Err = <Storage as std::str::FromStr>::Err;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Ok(Self {
                storage: <Storage as std::str::FromStr>::from_str(s)?,
                created: None,
            })
        }
    }

    /// Describes the storage format of an FBX file.
    #[derive(
        Default, Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr,
    )]
    #[display(style = "snake_case")]
    #[serde(rename = "FbxStorage", rename_all = "snake_case")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    pub enum Storage {
        /// ASCII FBX encoding.
        Ascii,

        /// Binary FBX encoding.
        #[default]
        Binary,
    }
}
