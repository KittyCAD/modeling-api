use bon::Builder;
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
    #[serde(
        serialize_with = "serde_bytes::serialize",
        deserialize_with = "serde_bytes::deserialize"
    )]
    pub contents: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Builder)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
/// Successful KCL project execution response.
pub struct ExecKclProjectOk {
    // TODO: Add fields to this as we make KCL data serializable.
    // Should be a usable subset of `SceneGraphDelta`.
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Builder)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
/// Failed KCL project execution response.
pub struct ExecKclProjectErr {
    /// Fatal KCL errors that prevented your geometry from being created.
    pub error: Option<KclError>,
    /// Nonfatal KCL errors that need to be fixed.
    pub non_fatal: Vec<CompilationIssue>,
    // TODO: Add fields to this as we make KCL data serializable.
    // Should be a usable subset of `KclErrorWithOutputs`.
}

#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for ExecKclProjectOk {
    fn arbitrary(_u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Self {})
    }
}

#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for ExecKclProjectErr {
    fn arbitrary(_u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Self {
            error: Default::default(),
            non_fatal: Default::default(),
        })
    }
}
