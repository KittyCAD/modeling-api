/// Export sketches in DXF format.
pub mod export {
    use bon::Builder;
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    /// Export storage.
    #[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, JsonSchema, PartialEq, Serialize)]
    #[serde(rename = "DxfStorage", rename_all = "snake_case")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    #[cfg_attr(
        feature = "python",
        pyo3_stub_gen::derive::gen_stub_pyclass_enum,
        pyo3::pyclass(name = "DxfStorage")
    )]
    #[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
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
    #[derive(Clone, Debug, Default, Deserialize, Eq, Hash, JsonSchema, PartialEq, Serialize, Builder)]
    #[serde(rename = "DxfExportOptions")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    #[cfg_attr(
        feature = "python",
        pyo3_stub_gen::derive::gen_stub_pyclass,
        pyo3::pyclass(name = "DxfExportOptions")
    )]
    #[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
    pub struct Options {
        /// Export storage.
        #[builder(default)]
        #[serde(default)]
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
