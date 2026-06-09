use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::shared::safe_filepath::SafeFilepath;

/// A KCL project that can be executed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default, Builder)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub struct KclProject {
    /// All files in the project.
    pub files: Vec<KclFile>,
    /// Which file is the entrypoint?
    /// This is the first KCL file to be executed,
    /// the root of the KCL module tree.
    pub entrypoint: SafeFilepath,
}

/// Region-creation algorithm version.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default, Builder)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub struct KclFile {
    /// Where is the file, relative to the project directory?
    pub path: SafeFilepath,
    /// Contents of the file, as UTF-8 encoded bytes.
    #[serde(deserialize_with = "serde_bytes::deserialize")]
    #[serde(serialize_with = "serde_bytes::serialize")]
    pub contents: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Builder)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
/// Successful KCL project execution response.
pub struct ExecKclProjectOk {
    /// Scene graph updates and execution metadata produced by running the KCL project.
    pub scene_graph_delta:
        kcl_api::SceneGraphDelta<kcl_api::SceneGraph<serde_json::Value, serde_json::Value>, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Builder)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
/// Failed KCL project execution response.
pub struct ExecKclProjectErr {
    /// Error produced while executing the KCL project.
    pub error: kcl_api::Error,
}

#[cfg(feature = "arbitrary")]
// TODO: Impl this properly for fuzzing.
impl<'a> arbitrary::Arbitrary<'a> for ExecKclProjectOk {
    fn arbitrary(_u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Self {
            scene_graph_delta: kcl_api::SceneGraphDelta::new(
                kcl_api::SceneGraph::empty(kcl_api::ProjectId(0), kcl_api::FileId(0), kcl_api::Version(0)),
                Vec::new(),
                false,
                serde_json::Value::Null,
            ),
        })
    }
}

#[cfg(feature = "arbitrary")]
// TODO: Impl this properly for fuzzing.
impl<'a> arbitrary::Arbitrary<'a> for ExecKclProjectErr {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Self {
            error: kcl_api::Error {
                msg: <String as arbitrary::Arbitrary>::arbitrary(u)?,
            },
        })
    }
}
