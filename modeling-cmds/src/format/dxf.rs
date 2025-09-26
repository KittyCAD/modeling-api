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
    #[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    #[cfg_attr(
        feature = "python",
        pyo3_stub_gen::derive::gen_stub_pyclass_enum,
        pyo3::pyclass(name = "DxfStorage")
    )]
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
    #[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    #[cfg_attr(
        feature = "python",
        pyo3_stub_gen::derive::gen_stub_pyclass,
        pyo3::pyclass(name = "DxfExportOptions")
    )]
    pub struct Options {
        /// Export storage.
        pub storage: Storage,
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
}
