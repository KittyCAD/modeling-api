use serde::{Deserialize, Serialize};

/// Data for an imported geometry.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImportedGeometry {
    /// The ID of the imported geometry.
    pub id: uuid::Uuid,
    /// The original file paths.
    pub value: Vec<String>,
}
