use crate::format::View;
use bon::Builder;
use parse_display::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Import models in KittyCAD's GLTF format.
pub mod import {
    use super::*;

    /// Options for importing glTF 2.0.
    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Builder)]
    #[serde(rename = "GltfImportOptions")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    #[cfg_attr(
        feature = "python",
        pyo3_stub_gen::derive::gen_stub_pyclass,
        pyo3::pyclass(name = "GltfImportOptions", from_py_object)
    )]
    #[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
    pub struct Options {}

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

/// Export models in KittyCAD's GLTF format.
pub mod export {
    use super::*;

    /// Options for exporting glTF 2.0.
    #[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, Builder)]
    #[serde(default, rename = "GltfExportOptions")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
    #[cfg_attr(
        feature = "python",
        pyo3_stub_gen::derive::gen_stub_pyclass,
        pyo3::pyclass(name = "GltfExportOptions", from_py_object)
    )]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    #[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
    pub struct Options {
        /// Specifies which kind of glTF 2.0 will be exported.
        #[builder(default)]
        pub storage: Storage,

        /// Specifies how the JSON will be presented.
        #[builder(default)]
        pub presentation: Presentation,

        /// Additional data to include in the exported glTF. Defaults to an empty list.
        #[builder(default)]
        pub include: Vec<Include>,

        /// Named views in the scene.
        #[builder(default)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        pub views: HashMap<String, View>,
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

    /// Additional data that can be included in a glTF 2.0 export.
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr)]
    #[display(style = "snake_case")]
    #[serde(rename = "GltfExportInclude", rename_all = "snake_case")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    #[cfg_attr(
        feature = "python",
        pyo3_stub_gen::derive::gen_stub_pyclass_enum,
        pyo3::pyclass(name = "GltfExportInclude", from_py_object)
    )]
    #[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
    pub enum Include {
        /// Sketch geometry and metadata.
        Sketches,
        /// Sketch regions and their resolver metadata.
        Regions,
        /// Original imported files.
        ImportedFiles,
        /// Model-based definition (MBD) annotations.
        Mbd,
    }

    /// Describes the storage format of a glTF 2.0 scene.
    #[derive(
        Default, Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr,
    )]
    #[display(style = "snake_case")]
    #[serde(rename = "GltfStorage", rename_all = "snake_case")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    #[cfg_attr(
        feature = "python",
        pyo3_stub_gen::derive::gen_stub_pyclass_enum,
        pyo3::pyclass(name = "GltfStorage", from_py_object)
    )]
    #[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
    pub enum Storage {
        /// Binary glTF 2.0.
        ///
        /// This is a single binary with .glb extension.
        Binary,

        /// Standard glTF 2.0.
        ///
        /// This is a JSON file with .gltf extension paired with a separate
        /// binary blob file with .bin extension.
        Standard,

        /// Embedded glTF 2.0.
        ///
        /// Single JSON file with .gltf extension binary data encoded as
        /// base64 data URIs.
        ///
        /// This is the default setting.
        #[default]
        Embedded,
    }

    /// Describes the presentation style of the glTF JSON.
    #[derive(
        Default, Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr,
    )]
    #[display(style = "snake_case")]
    #[serde(rename = "GltfPresentation", rename_all = "snake_case")]
    #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
    #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
    #[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
    #[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
    pub enum Presentation {
        /// Condense the JSON into the smallest possible size.
        Compact,

        /// Expand the JSON into a more human readable format.
        ///
        /// This is the default setting.
        #[default]
        Pretty,
    }
}

#[cfg(test)]
mod tests {
    use super::export::{Include, Options, Presentation, Storage};
    use crate::format::OutputFormat3d;

    #[test]
    fn additional_data_is_opt_in() {
        let options: Options = serde_json::from_str(r#"{"storage":"binary","presentation":"compact"}"#).unwrap();
        assert_eq!(
            options,
            Options::builder()
                .storage(Storage::Binary)
                .presentation(Presentation::Compact)
                .build()
        );
        assert!(options.include.is_empty());
        assert!(Options::default().include.is_empty());
        assert_eq!(Options::builder().build(), Options::default());
        assert_eq!(serde_json::from_str::<Options>("{}").unwrap(), Options::default());
        assert_eq!(serde_json::to_value(options).unwrap()["include"], serde_json::json!([]));
    }

    #[test]
    fn include_options_round_trip() {
        let selections = [
            (Include::Sketches, "sketches"),
            (Include::Regions, "regions"),
            (Include::ImportedFiles, "imported_files"),
            (Include::Mbd, "mbd"),
        ];

        // Each category can be selected independently or combined with any others.
        for mask in 0..(1 << selections.len()) {
            let selected: Vec<_> = selections
                .iter()
                .enumerate()
                .filter(|(index, _)| mask & (1 << index) != 0)
                .map(|(_, selection)| *selection)
                .collect();
            let include: Vec<_> = selected.iter().map(|(include, _)| *include).collect();
            let names: Vec<_> = selected.iter().map(|(_, name)| *name).collect();
            let json = serde_json::json!({
                "storage": "binary",
                "presentation": "compact",
                "include": names
            });
            let options: Options = serde_json::from_value(json.clone()).unwrap();
            assert_eq!(
                options,
                Options::builder()
                    .storage(Storage::Binary)
                    .presentation(Presentation::Compact)
                    .include(include)
                    .build()
            );
            assert_eq!(serde_json::to_value(options).unwrap(), json);
        }
    }

    #[test]
    fn include_options_round_trip_in_output_format() {
        let json = serde_json::json!({
            "type": "gltf",
            "storage": "binary",
            "presentation": "compact",
            "include": ["sketches", "regions", "imported_files", "mbd"]
        });
        let format: OutputFormat3d = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(
            format,
            OutputFormat3d::Gltf(
                Options::builder()
                    .storage(Storage::Binary)
                    .presentation(Presentation::Compact)
                    .include(vec![
                        Include::Sketches,
                        Include::Regions,
                        Include::ImportedFiles,
                        Include::Mbd
                    ])
                    .build()
            )
        );
        assert_eq!(serde_json::to_value(format).unwrap(), json);
    }

    #[test]
    fn invalid_include_options_are_rejected() {
        for include in [
            serde_json::json!(["unknown"]),
            serde_json::json!(["uuids"]),
            serde_json::json!(["MBD"]),
            serde_json::json!("sketches"),
            serde_json::json!(true),
            serde_json::json!(null),
        ] {
            let json = serde_json::json!({"include": include});
            assert!(serde_json::from_value::<Options>(json).is_err());
        }
    }
}
