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

        /// Include engine UUIDs in glTF extras. Defaults to false.
        #[builder(default)]
        pub include_uuids: bool,

        /// Tessellation settings for converting CAD surfaces to triangles.
        /// Omit to preserve the current tessellation behavior. Existing meshes are unchanged.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tessellation: Option<crate::format::tessellation::Options>,

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
    use super::export::{Options, Presentation, Storage};
    use crate::format::tessellation::{
        LevelOfDetail as TessellationLevelOfDetail, Mode, Options as TessellationOptions,
    };

    #[test]
    fn uuid_metadata_is_opt_in() {
        let options: Options = serde_json::from_str(r#"{"storage":"binary","presentation":"compact"}"#).unwrap();
        assert_eq!(
            options,
            Options::builder()
                .storage(Storage::Binary)
                .presentation(Presentation::Compact)
                .build()
        );
        assert!(!options.include_uuids);
        assert!(!Options::default().include_uuids);
        assert_eq!(Options::builder().build(), Options::default());
    }

    #[test]
    fn uuid_option_round_trips() {
        for include_uuids in [false, true] {
            let json = serde_json::json!({
                "storage": "binary",
                "presentation": "compact",
                "include_uuids": include_uuids
            });
            let options: Options = serde_json::from_value(json.clone()).unwrap();
            assert_eq!(options.include_uuids, include_uuids);
            assert_eq!(serde_json::to_value(options).unwrap(), json);
        }
    }

    #[test]
    fn tessellation_is_opt_in() {
        let options: Options = serde_json::from_str("{}").unwrap();
        assert_eq!(options, Options::default());
        assert_eq!(Options::builder().build(), options);
        assert!(options.tessellation.is_none());
        assert_eq!(
            serde_json::to_value(options).unwrap(),
            serde_json::json!({
                "storage": "embedded",
                "presentation": "pretty",
                "include_uuids": false
            })
        );
    }

    #[test]
    fn tessellation_partial_objects_use_defaults() {
        let defaults = TessellationOptions::default();
        assert_eq!(defaults.level_of_detail, TessellationLevelOfDetail::High);
        assert_eq!(defaults.mode, Mode::Accurate);
        assert_eq!(TessellationOptions::builder().build(), defaults);

        for (json, expected) in [
            (serde_json::json!({}), defaults),
            (
                serde_json::json!({"level_of_detail": "medium"}),
                TessellationOptions::builder()
                    .level_of_detail(TessellationLevelOfDetail::Medium)
                    .build(),
            ),
            (
                serde_json::json!({"mode": "visual"}),
                TessellationOptions::builder().mode(Mode::Visual).build(),
            ),
        ] {
            let options: Options = serde_json::from_value(serde_json::json!({"tessellation": json})).unwrap();
            assert_eq!(options.tessellation, Some(expected));
        }
    }

    #[test]
    fn tessellation_options_round_trip() {
        for level in ["extra_low", "low", "medium", "high", "extra_high"] {
            let json = serde_json::json!({
                "storage": "binary",
                "presentation": "compact",
                "include_uuids": true,
                "tessellation": {
                    "level_of_detail": level,
                    "mode": "visual"
                }
            });
            let options: Options = serde_json::from_value(json.clone()).unwrap();
            assert_eq!(serde_json::to_value(options).unwrap(), json);
        }
    }

    #[test]
    fn tessellation_rejects_unknown_detail_levels() {
        assert!(serde_json::from_value::<Options>(serde_json::json!({
            "tessellation": {"level_of_detail": "ultra"}
        }))
        .is_err());
    }

    #[test]
    fn visualization_tessellation_selects_visual_mode() {
        assert_eq!(
            TessellationOptions::visualization(),
            TessellationOptions::builder().mode(Mode::Visual).build()
        );
    }
}
