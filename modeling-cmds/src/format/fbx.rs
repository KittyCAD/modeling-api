use crate::datetime::DateTimeLocal;
use kittycad_execution_plan_macros::ExecutionPlanValue;
use parse_display::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Import models in FBX format.
pub mod import {
    use super::*;
    /// Options for importing FBX.
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        Hash,
        PartialEq,
        Serialize,
        Deserialize,
        JsonSchema,
        Display,
        FromStr,
        ExecutionPlanValue,
    )]
    #[display("")]
    #[serde(rename = "FbxImportOptions")]
    pub struct Options {}
}

/// Export models in FBX format.
pub mod export {
    use super::*;

    /// Options for exporting FBX.
    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
    #[serde(rename = "FbxExportOptions")]
    pub struct Options {
        /// Specifies which kind of FBX will be exported.
        pub storage: Storage,

        /// Timestamp override.
        ///
        /// This is intended for local integration testing only; it is not provided as an option
        /// in the JSON schema.
        #[serde(skip)]
        pub created: Option<DateTimeLocal>,
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
        Default,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        PartialEq,
        Serialize,
        Deserialize,
        JsonSchema,
        Display,
        FromStr,
        ExecutionPlanValue,
    )]
    #[display(style = "snake_case")]
    #[serde(rename = "FbxStorage", rename_all = "snake_case")]
    pub enum Storage {
        /// ASCII FBX encoding.
        Ascii,

        /// Binary FBX encoding.
        #[default]
        Binary,
    }
}
