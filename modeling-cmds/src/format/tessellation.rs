use bon::Builder;
use parse_display::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Controls mesh generation for mesh exports.
#[derive(Default, Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Builder)]
#[serde(default, rename = "TessellationOptions")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(
    feature = "python",
    pyo3_stub_gen::derive::gen_stub_pyclass,
    pyo3::pyclass(name = "TessellationOptions", from_py_object)
)]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub struct Options {
    /// Tessellation mode. Defaults to accurate.
    #[builder(default)]
    pub mode: Mode,
    /// Detail level of the generated triangles. Defaults to high.
    #[builder(default)]
    pub level_of_detail: LevelOfDetail,
}

impl Options {
    /// Use high-detail visualization tessellation.
    pub fn visualization() -> Self {
        Self {
            mode: Mode::Visual,
            ..Self::default()
        }
    }
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

/// Selects the purpose of the generated mesh.
#[derive(Default, Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr)]
#[display(style = "snake_case")]
#[serde(rename = "TessellationMode", rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(
    feature = "python",
    pyo3_stub_gen::derive::gen_stub_pyclass_enum,
    pyo3::pyclass(name = "TessellationMode", from_py_object)
)]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub enum Mode {
    /// Generate a mesh for visualization.
    Visual,
    /// Generate a mesh with the accurate tessellation settings.
    #[default]
    Accurate,
}
/// Detail level used when generating triangles from CAD surfaces.
#[derive(Default, Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr)]
#[display(style = "snake_case")]
#[serde(rename = "TessellationLevelOfDetail", rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(
    feature = "python",
    pyo3_stub_gen::derive::gen_stub_pyclass_enum,
    pyo3::pyclass(name = "TessellationLevelOfDetail", from_py_object)
)]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub enum LevelOfDetail {
    /// Extra-low detail.
    ExtraLow,
    /// Low detail.
    Low,
    /// Medium detail.
    Medium,
    /// High detail. This is the default.
    #[default]
    High,
    /// Extra-high detail.
    ExtraHigh,
}

#[cfg(test)]
mod tests {
    use super::{Mode, Options};
    use crate::format::OutputFormat3d;

    #[test]
    fn mesh_formats_share_optional_tessellation_modes() {
        for format in [
            OutputFormat3d::Fbx(Default::default()),
            OutputFormat3d::Gltf(Default::default()),
            OutputFormat3d::Obj(Default::default()),
            OutputFormat3d::Ply(Default::default()),
            OutputFormat3d::Stl(Default::default()),
        ] {
            let original = serde_json::to_value(&format).unwrap();
            assert!(original.get("tessellation").is_none());
            let restored: OutputFormat3d = serde_json::from_value(original.clone()).unwrap();
            assert_eq!(restored, format);
            assert_eq!(restored.tessellation(), None);
            for (settings, expected) in [
                (serde_json::json!({}), Mode::Accurate),
                (serde_json::json!({"mode": "accurate"}), Mode::Accurate),
                (serde_json::json!({"mode": "visual"}), Mode::Visual),
            ] {
                let mut request = original.clone();
                request["tessellation"] = settings;
                let parsed: OutputFormat3d = serde_json::from_value(request).unwrap();
                assert_eq!(parsed.tessellation().unwrap().mode, expected);
                let serialized = serde_json::to_value(&parsed).unwrap();
                assert_eq!(serialized["tessellation"]["mode"], expected.to_string());
                assert_eq!(serde_json::from_value::<OutputFormat3d>(serialized).unwrap(), parsed);
            }
        }
        assert_eq!(Options::default().mode, Mode::Accurate);
        assert_eq!(Options::builder().build(), Options::default());
        assert!(serde_json::from_value::<Options>(serde_json::json!({"mode": "unknown"})).is_err());
    }
}
