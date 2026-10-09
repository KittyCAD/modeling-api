use crate::shared::safe_filepath::SafeFilepath;
use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A KCL project.
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

/// A file in a KCL project.
#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default, Builder)]
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
