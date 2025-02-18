use parse_display_derive::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::shared::{FileExportFormat, FileExportFormat2d, FileImportFormat};

/// AutoCAD drawing interchange format.
pub mod dxf;
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

/// SolidWorks part (SLDPRT) format.
pub mod sldprt;

/// Output 2D format specifier.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr)]
#[serde(tag = "type", rename_all = "snake_case")]
#[display(style = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
pub enum OutputFormat2d {
    /// AutoCAD drawing interchange format.
    #[display("{}: {0}")]
    Dxf(dxf::export::Options),
}

/// Alias for backward compatibility.
#[deprecated(since = "0.2.96", note = "use `OutputFormat3d` instead")]
pub type OutputFormat = OutputFormat3d;

/// Output 3D format specifier.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr)]
#[serde(tag = "type", rename_all = "snake_case")]
#[display(style = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
pub enum OutputFormat3d {
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
    Step(step::export::Options),
    /// **ST**ereo**L**ithography format.
    #[display("{}: {0}")]
    Stl(stl::export::Options),
}

/// Input format specifier.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr)]
#[serde(tag = "type", rename_all = "snake_case")]
#[display(style = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
pub enum InputFormat {
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
    /// SolidWorks part (SLDPRT) format.
    #[display("{}: {0}")]
    Sldprt(sldprt::import::Options),
    /// ISO 10303-21 (STEP) format.
    #[display("{}: {0}")]
    Step(step::import::Options),
    /// **ST**ereo**L**ithography format.
    #[display("{}: {0}")]
    Stl(stl::import::Options),
}

/// Data item selection.
#[derive(Clone, Debug, Default, Display, Eq, FromStr, Hash, PartialEq, JsonSchema, Deserialize, Serialize)]
#[display(style = "snake_case")]
#[serde(rename_all = "snake_case", tag = "type")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
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

impl From<OutputFormat3d> for FileExportFormat {
    fn from(output_format: OutputFormat3d) -> Self {
        match output_format {
            OutputFormat3d::Fbx(_) => Self::Fbx,
            OutputFormat3d::Gltf(_) => Self::Gltf,
            OutputFormat3d::Obj(_) => Self::Obj,
            OutputFormat3d::Ply(_) => Self::Ply,
            OutputFormat3d::Step(_) => Self::Step,
            OutputFormat3d::Stl(_) => Self::Stl,
        }
    }
}

impl From<OutputFormat2d> for FileExportFormat2d {
    fn from(output_format: OutputFormat2d) -> Self {
        match output_format {
            OutputFormat2d::Dxf(_) => Self::Dxf,
        }
    }
}

impl From<FileExportFormat2d> for OutputFormat2d {
    fn from(export_format: FileExportFormat2d) -> Self {
        match export_format {
            FileExportFormat2d::Dxf => OutputFormat2d::Dxf(Default::default()),
        }
    }
}

impl From<FileExportFormat> for OutputFormat3d {
    fn from(export_format: FileExportFormat) -> Self {
        match export_format {
            FileExportFormat::Fbx => OutputFormat3d::Fbx(Default::default()),
            FileExportFormat::Glb => OutputFormat3d::Gltf(gltf::export::Options {
                storage: gltf::export::Storage::Binary,
                ..Default::default()
            }),
            FileExportFormat::Gltf => OutputFormat3d::Gltf(gltf::export::Options {
                storage: gltf::export::Storage::Embedded,
                presentation: gltf::export::Presentation::Pretty,
            }),
            FileExportFormat::Obj => OutputFormat3d::Obj(Default::default()),
            FileExportFormat::Ply => OutputFormat3d::Ply(Default::default()),
            FileExportFormat::Step => OutputFormat3d::Step(Default::default()),
            FileExportFormat::Stl => OutputFormat3d::Stl(stl::export::Options {
                storage: stl::export::Storage::Ascii,
                ..Default::default()
            }),
        }
    }
}

impl From<InputFormat> for FileImportFormat {
    fn from(input_format: InputFormat) -> Self {
        match input_format {
            InputFormat::Fbx(_) => Self::Fbx,
            InputFormat::Gltf(_) => Self::Gltf,
            InputFormat::Obj(_) => Self::Obj,
            InputFormat::Ply(_) => Self::Ply,
            InputFormat::Sldprt(_) => Self::Sldprt,
            InputFormat::Step(_) => Self::Step,
            InputFormat::Stl(_) => Self::Stl,
        }
    }
}

impl From<FileImportFormat> for InputFormat {
    fn from(import_format: FileImportFormat) -> Self {
        match import_format {
            FileImportFormat::Fbx => InputFormat::Fbx(Default::default()),
            FileImportFormat::Gltf => InputFormat::Gltf(Default::default()),
            FileImportFormat::Obj => InputFormat::Obj(Default::default()),
            FileImportFormat::Ply => InputFormat::Ply(Default::default()),
            FileImportFormat::Sldprt => InputFormat::Sldprt(Default::default()),
            FileImportFormat::Step => InputFormat::Step(Default::default()),
            FileImportFormat::Stl => InputFormat::Stl(Default::default()),
        }
    }
}
