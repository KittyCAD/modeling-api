/// Import functionality.
pub mod import {

    use bon::Builder;
    use parse_display::{Display, FromStr};
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    /// Options for importing SolidWorks parts.
    #[derive(
        Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr, Builder,
    )]
    #[display("split_closed_faces: {split_closed_faces}")]
    #[serde(default, rename = "SldprtImportOptions")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    #[cfg_attr(
        feature = "python",
        pyo3_stub_gen::derive::gen_stub_pyclass,
        pyo3::pyclass(name = "SldprtImportOptions")
    )]
    pub struct Options {
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
}
