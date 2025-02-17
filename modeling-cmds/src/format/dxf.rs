/// Export sketches in DXF format.
pub mod export {
    use parse_display::{Display, FromStr};
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    /// Options for exporting DXF format.
    #[derive(Clone, Debug, Default, Deserialize, Display, Eq, FromStr, Hash, JsonSchema, PartialEq, Serialize)]
    #[display("")]
    #[serde(rename = "DxfExportOptions")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    pub struct Options {}
}
