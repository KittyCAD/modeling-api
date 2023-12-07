use parse_display_derive::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Autodesk Filmbox (FBX) format.
pub mod fbx;
/// glTF 2.0.
/// We refer to this as glTF since that is how our customers refer to it, although by default
/// it will be in binary format and thus technically (glb).
/// If you prefer ASCII output, you can set that option for the export.
pub mod gltf;
/// Wavefront OBJ format.
pub mod obj;
/// The PLY Polygon File Format.
pub mod ply;
/// ISO 10303-21 (STEP) format.
pub mod step;
/// **ST**ereo**L**ithography format.
pub mod stl;

/// Output format specifier.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr)]
#[serde(tag = "type", rename_all = "snake_case")]
#[display(style = "snake_case")]
pub enum OutputFormat {
    // TODO: Uncomment all these variants and support their options.
    /// Autodesk Filmbox (FBX) format.
    #[display("{}: {0}")]
    Fbx(fbx::export::Options),
    /// glTF 2.0.
    /// We refer to this as glTF since that is how our customers refer to it, although by default
    /// it will be in binary format and thus technically (glb).
    /// If you prefer ASCII output, you can set that option for the export.
    #[display("{}: {0}")]
    Gltf(gltf::export::Options),
    /// Wavefront OBJ format.
    #[display("{}: {0}")]
    Obj(obj::export::Options),
    /// The PLY Polygon File Format.
    #[display("{}: {0}")]
    Ply(ply::export::Options),
    /// ISO 10303-21 (STEP) format.
    #[display("{}: {0}")]
    Step(step::ExportOptions),
    /// **ST**ereo**L**ithography format.
    #[display("{}: {0}")]
    Stl(stl::export::Options),
}

/// Input format specifier.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr)]
#[serde(tag = "type", rename_all = "snake_case")]
#[display(style = "snake_case")]
pub enum InputFormat {
    // TODO: Uncomment all these variants and support their options.
    /// Autodesk Filmbox (FBX) format.
    #[display("{}: {0}")]
    Fbx(fbx::import::Options),
    /// Binary glTF 2.0.
    /// We refer to this as glTF since that is how our customers refer to it,
    /// but this can also import binary glTF (glb).
    #[display("{}: {0}")]
    Gltf(gltf::import::Options),
    /// Wavefront OBJ format.
    #[display("{}: {0}")]
    Obj(obj::import::Options),
    /// The PLY Polygon File Format.
    #[display("{}: {0}")]
    Ply(ply::import::Options),
    // /// SolidWorks part (SLDPRT) format.
    // #[display("{}: {0}")]
    // Sldprt(sldprt::import::Options),
    /// ISO 10303-21 (STEP) format.
    #[display("{}: {0}")]
    Step(step::ImportOptions),
    /// **ST**ereo**L**ithography format.
    #[display("{}: {0}")]
    Stl(stl::import::Options),
}

/// Data item selection.
#[derive(Clone, Debug, Default, Display, Eq, FromStr, Hash, PartialEq, JsonSchema, Deserialize, Serialize)]
#[display(style = "snake_case")]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Selection {
    /// Visit the default scene.
    #[default]
    DefaultScene,

    /// Visit the indexed scene.
    #[display("{}: {index}")]
    SceneByIndex {
        /// The index.
        index: usize,
    },

    /// Visit the first scene with the given name.
    #[display("{}: {name}")]
    SceneByName {
        /// The name.
        name: String,
    },

    /// Visit the indexed mesh.
    #[display("{}: {index}")]
    MeshByIndex {
        /// The index.
        index: usize,
    },

    /// Visit the first mesh with the given name.
    #[display("{}: {name}")]
    MeshByName {
        /// The name.
        name: String,
    },
}

/// Represents an in-memory file with an associated potentially foreign file path.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VirtualFile {
    /// Original file path.
    pub path: std::path::PathBuf,
    /// File payload.
    pub data: Vec<u8>,
}

impl VirtualFile {
    /// Returns true if the file name has the given extension.
    pub fn has_extension(&self, required_extension: &str) -> bool {
        self.path
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .map(|extension| extension.eq_ignore_ascii_case(required_extension))
            .unwrap_or(false)
    }

    fn read_fs_impl(path: std::path::PathBuf) -> std::io::Result<Self> {
        let data = std::fs::read(&path)?;
        Ok(Self { path, data })
    }

    /// Read from file system.
    pub fn read_fs<P>(path: P) -> std::io::Result<Self>
    where
        P: Into<std::path::PathBuf>,
    {
        Self::read_fs_impl(path.into())
    }
}
