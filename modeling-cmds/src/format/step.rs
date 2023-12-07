use crate::coord;
use parse_display::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Options for exporting STEP format.
#[derive(Clone, Debug, Deserialize, Eq, Hash, JsonSchema, PartialEq, Serialize)]
#[serde(rename = "StepExportOptions")]
pub struct ExportOptions {
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

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            coords: *coord::KITTYCAD,
            created: None,
        }
    }
}

impl std::fmt::Display for ExportOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "coords: {}", self.coords)
    }
}

impl std::str::FromStr for ExportOptions {
    type Err = <coord::System as std::str::FromStr>::Err;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self {
            coords: <coord::System as std::str::FromStr>::from_str(s)?,
            created: None,
        })
    }
}

/// Options for importing STEP format.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr)]
#[display("")]
#[serde(rename = "StepImportOptions")]
pub struct ImportOptions {}
