/// Export sketches in DXF format.
pub mod export {
    use parse_display::{Display, FromStr};
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    /// Export storage.
    #[derive(
        Clone, Copy, Debug, Default, Deserialize, Display, Eq, FromStr, Hash, JsonSchema, PartialEq, Serialize,
    )]
    #[display(style = "snake_case")]
    #[serde(rename = "DxfStorage", rename_all = "snake_case")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    pub enum Storage {
        /// Plaintext encoding.
        ///
        /// This is the default setting.
        #[default]
        Ascii,

        /// Binary encoding.
        Binary,
    }

    /// Options for exporting DXF format.
    #[derive(Clone, Debug, Default, Deserialize, Display, Eq, FromStr, Hash, JsonSchema, PartialEq, Serialize)]
    #[display("storage: {storage}")]
    #[serde(rename = "DxfExportOptions")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    #[cfg_attr(feature = "python", pyo3::pyclass)]
    pub struct Options {
        /// Export storage.
        pub storage: Storage,
    }
}
