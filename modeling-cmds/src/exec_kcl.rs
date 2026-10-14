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
    /// Include interpreter data needed for saved diagnostics and sketch rendering.
    /// Clients requesting this must verify the response contains execution_data.
    #[serde(default)]
    #[builder(default)]
    pub include_execution_data: bool,
}

impl KclProject {
    /// Create a new KCL project.
    pub fn new(files: Vec<KclFile>, entrypoint: SafeFilepath) -> Self {
        Self {
            files,
            entrypoint,
            include_execution_data: false,
        }
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
    /// Original client path, used only as a name in the uploaded in-memory filesystem.
    /// This can be absolute or contain parent components; it never grants access
    /// to the engine's filesystem. Preserves import resolution and diagnostic names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-rs", ts(optional))]
    pub original_path: Option<String>,
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
        Self {
            path,
            contents,
            original_path: None,
        }
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
    /// Interpreter-specific data for saved source diagnostics and sketch rendering.
    /// Contains solved scene objects and module sources; its schema follows the deployed
    /// interpreter release, just like KCL execution semantics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-rs", ts(optional))]
    pub execution_data: Option<serde_json::Value>,
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
    /// Interpreter-specific data for saved source diagnostics and sketch rendering.
    /// Contains solved scene objects and module sources; its schema follows the deployed
    /// interpreter release, just like KCL execution semantics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-rs", ts(optional))]
    pub execution_data: Option<serde_json::Value>,
    /// All objects in all sketches.
    pub scene_objects: Vec<Object>,
    /// Which module ID was assigned to each KCL file in the project?
    pub filenames: IndexMap<kcl_error::ModuleId, kcl_api::modules::ModulePath>,
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
            execution_data: None,
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
            execution_data: None,
        })
    }
}

#[cfg(all(test, feature = "websocket"))]
mod tests {
    use super::*;

    #[test]
    fn existing_projects_deserialize_without_native_metadata() {
        let project: KclProject = serde_json::from_value(serde_json::json!({
            "files": [{"path": "main.kcl", "contents": [120]}],
            "entrypoint": "main.kcl"
        }))
        .unwrap();
        assert!(!project.include_execution_data);
        assert!(project.files[0].original_path.is_none());
        assert_eq!(project.files[0].contents, b"x");
    }

    #[test]
    fn native_project_metadata_and_binary_assets_round_trip() {
        let mut file = KclFile::new(SafeFilepath::validate("files/0").unwrap(), vec![0, 255, 128]);
        file.original_path = Some("/client/project/../part.step".to_owned());
        let mut project = KclProject::new(vec![file], SafeFilepath::validate("files/0").unwrap());
        project.include_execution_data = true;
        let saved: KclProject = serde_json::from_slice(&serde_json::to_vec(&project).unwrap()).unwrap();
        assert_eq!(saved, project);
    }

    #[test]
    fn execution_data_is_optional_on_success_and_failure_responses() {
        let success = ExecKclProjectOk::builder()
            .artifact_graph(Default::default())
            .operations(Default::default())
            .variables(Default::default())
            .issues(vec![])
            .build();
        let success_json = serde_json::to_value(success).unwrap();
        assert!(!success_json.as_object().unwrap().contains_key("execution_data"));
        let success: ExecKclProjectOk = serde_json::from_value(success_json).unwrap();
        assert!(success.execution_data.is_none());
        let failure = ExecKclProjectErr::builder()
            .artifact_graph(Default::default())
            .operations(Default::default())
            .variables(Default::default())
            .non_fatal(vec![])
            .filenames(Default::default())
            .scene_objects(Default::default())
            .build();
        let failure_json = serde_json::to_value(failure).unwrap();
        assert!(!failure_json.as_object().unwrap().contains_key("execution_data"));
        let failure: ExecKclProjectErr = serde_json::from_value(failure_json).unwrap();
        assert!(failure.execution_data.is_none());
    }
}
