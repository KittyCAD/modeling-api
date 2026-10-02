use bon::Builder;
use indexmap::IndexMap;
#[cfg(feature = "websocket")]
use kcl_api::ArtifactGraph;
use kcl_api::{kcl_value_view::KclValueView, DefaultPlanes};
use kcl_error::{CompilationIssue, KclError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use crate::shared::{KclFile, KclProject};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Builder)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
/// Successful KCL project execution response.
pub struct ExecKclProjectOk {
    /// The artifact graph produced by the KCL execution.
    #[cfg(feature = "websocket")]
    pub artifact_graph: ArtifactGraph,
    /// Operations that have been performed in execution order, grouped by
    /// owning module id, for display in the Feature Tree.
    #[cfg(feature = "websocket")]
    pub operations: IndexMap<kcl_error::ModuleId, Vec<kcl_api::Operation>>,
    /// Variables in the top-level of the root module.
    pub variables: IndexMap<String, KclValueView>,
    /// Non-fatal errors and warnings.
    pub issues: Vec<CompilationIssue>,
    /// IDs of the standard planes created for this execution.
    pub default_planes: Option<Box<DefaultPlanes>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Builder)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
/// Failed KCL project execution response.
pub struct ExecKclProjectErr {
    /// Fatal KCL errors that prevented your geometry from being created.
    pub error: Option<KclError>,
    /// Operations that have been performed in execution order, grouped by
    /// owning module id, for display in the Feature Tree.
    #[cfg(feature = "websocket")]
    pub operations: IndexMap<kcl_error::ModuleId, Vec<kcl_api::Operation>>,
    /// Variables in the top-level of the root module.
    pub variables: IndexMap<String, KclValueView>,
    /// Non-fatal errors and warnings.
    pub non_fatal: Vec<CompilationIssue>,
    /// The artifact graph produced by the KCL execution.
    #[cfg(feature = "websocket")]
    pub artifact_graph: ArtifactGraph,
    /// IDs of the standard planes created before execution failed.
    pub default_planes: Option<Box<DefaultPlanes>>,
    // TODO: Add fields to this as we make KCL data serializable.
    // Should be a usable subset of `KclErrorWithOutputs`.
}

impl ExecKclProjectErr {
    /// Used when the project execution had a fatal error.
    pub fn fatal_error(error: KclError) -> Self {
        Self {
            error: Some(error),
            non_fatal: Default::default(),
            operations: Default::default(),
            variables: Default::default(),
            artifact_graph: Default::default(),
            default_planes: Default::default(),
        }
    }
}

#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for ExecKclProjectOk {
    fn arbitrary(_u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Self {
            #[cfg(feature = "websocket")]
            artifact_graph: ArtifactGraph::default(),
            #[cfg(feature = "websocket")]
            operations: Default::default(),
            #[cfg(feature = "websocket")]
            issues: Default::default(),
            #[cfg(feature = "websocket")]
            variables: Default::default(),
            default_planes: Default::default(),
        })
    }
}

#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for ExecKclProjectErr {
    fn arbitrary(_u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Self {
            error: Default::default(),
            non_fatal: Default::default(),
            operations: Default::default(),
            variables: Default::default(),
            artifact_graph: Default::default(),
            default_planes: Default::default(),
        })
    }
}
