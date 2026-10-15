use bon::Builder;
use indexmap::IndexMap;
#[cfg(feature = "websocket")]
use kcl_api::artifact::ArtifactGraph;
use kcl_api::{kcl_value_view::KclValueView, DefaultPlanes, Object};
use kcl_error::{CompilationIssue, KclError};
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

impl KclProject {
    /// Create a new KCL project.
    pub fn new(files: Vec<KclFile>, entrypoint: SafeFilepath) -> Self {
        Self { files, entrypoint }
    }
}

/// A file uploaded with a KCL project.
#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default, Builder)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub struct KclFile {
    /// Where is the file, relative to the project directory?
    pub path: SafeFilepath,
    /// Raw file contents. KCL source is UTF-8; imported assets may be binary.
    #[serde(
        serialize_with = "serde_bytes::serialize",
        deserialize_with = "serde_bytes::deserialize"
    )]
    pub contents: Vec<u8>,
}

impl std::fmt::Debug for KclFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KclFile")
            .field("path", &self.path)
            .field("contents.len()", &self.contents.len())
            .finish()
    }
}

impl KclFile {
    /// Create a KCL file.
    pub fn new(path: SafeFilepath, contents: Vec<u8>) -> Self {
        Self { path, contents }
    }
}

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
    /// All objects in all sketches.
    pub scene_objects: Vec<Object>,
    /// Which module ID was assigned to each KCL file in the project?
    pub filenames: IndexMap<kcl_error::ModuleId, kcl_api::modules::ModulePath>,
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
    /// All objects in all sketches.
    pub scene_objects: Vec<Object>,
    /// Which module ID was assigned to each KCL file in the project?
    pub filenames: IndexMap<kcl_error::ModuleId, kcl_api::modules::ModulePath>,
}

impl ExecKclProjectErr {
    /// Be careful using this. If you have executed a KCL program,
    /// you should be building the full ExecKclProjectErr via its `build()`
    /// method. This is only for errors that happen before execution.
    pub fn error_before_execution(error: KclError) -> Self {
        Self {
            error: Some(error),
            non_fatal: Default::default(),
            operations: Default::default(),
            variables: Default::default(),
            artifact_graph: Default::default(),
            default_planes: Default::default(),
            filenames: Default::default(),
            scene_objects: Default::default(),
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
            filenames: Default::default(),
            scene_objects: Default::default(),
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
            scene_objects: Default::default(),
            filenames: Default::default(),
        })
    }
}
