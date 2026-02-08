use bon::Builder;
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
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
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
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
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

/// Alias for backward compatibility.
#[deprecated(since = "0.2.96", note = "use `InputFormat3d` instead")]
pub type InputFormat = InputFormat3d;

/// Input format specifier.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr)]
#[serde(tag = "type", rename_all = "snake_case")]
#[display(style = "snake_case")]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass,
    pyo3_stub_gen::derive::gen_stub_pyclass_complex_enum
)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub enum InputFormat3d {
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

impl InputFormat3d {
    /// Get the name of this format.
    pub fn name(&self) -> &'static str {
        match self {
            InputFormat3d::Fbx(_) => "fbx",
            InputFormat3d::Gltf(_) => "gltf",
            InputFormat3d::Obj(_) => "obj",
            InputFormat3d::Ply(_) => "ply",
            InputFormat3d::Sldprt(_) => "sldprt",
            InputFormat3d::Step(_) => "step",
            InputFormat3d::Stl(_) => "stl",
        }
    }
}

/// Data item selection.
#[derive(Clone, Debug, Default, Display, Eq, FromStr, Hash, PartialEq, JsonSchema, Deserialize, Serialize)]
#[display(style = "snake_case")]
#[serde(rename_all = "snake_case", tag = "type")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
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
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Builder)]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
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

impl From<InputFormat3d> for FileImportFormat {
    fn from(input_format: InputFormat3d) -> Self {
        match input_format {
            InputFormat3d::Fbx(_) => Self::Fbx,
            InputFormat3d::Gltf(_) => Self::Gltf,
            InputFormat3d::Obj(_) => Self::Obj,
            InputFormat3d::Ply(_) => Self::Ply,
            InputFormat3d::Sldprt(_) => Self::Sldprt,
            InputFormat3d::Step(_) => Self::Step,
            InputFormat3d::Stl(_) => Self::Stl,
        }
    }
}

impl From<FileImportFormat> for InputFormat3d {
    fn from(import_format: FileImportFormat) -> Self {
        match import_format {
            FileImportFormat::Fbx => InputFormat3d::Fbx(Default::default()),
            FileImportFormat::Gltf => InputFormat3d::Gltf(Default::default()),
            FileImportFormat::Obj => InputFormat3d::Obj(Default::default()),
            FileImportFormat::Ply => InputFormat3d::Ply(Default::default()),
            FileImportFormat::Sldprt => InputFormat3d::Sldprt(Default::default()),
            FileImportFormat::Step => InputFormat3d::Step(Default::default()),
            FileImportFormat::Stl => InputFormat3d::Stl(Default::default()),
        }
    }
}

/// Options for a 3D export.
pub struct OutputFormat3dOptions {
    src_unit: crate::units::UnitLength,
}

impl OutputFormat3dOptions {
    /// Create the options, setting all optional fields to their defaults.
    pub fn new(src_unit: crate::units::UnitLength) -> Self {
        Self { src_unit }
    }
}

impl OutputFormat3d {
    /// Create the output format, setting the options as given.
    pub fn new(format: &FileExportFormat, options: OutputFormat3dOptions) -> Self {
        let OutputFormat3dOptions { src_unit } = options;
        // Zoo co-ordinate system.
        //
        // * Forward: -Y
        // * Up: +Z
        // * Handedness: Right
        let coords = crate::coord::System {
            forward: crate::coord::AxisDirectionPair {
                axis: crate::coord::Axis::Y,
                direction: crate::coord::Direction::Negative,
            },
            up: crate::coord::AxisDirectionPair {
                axis: crate::coord::Axis::Z,
                direction: crate::coord::Direction::Positive,
            },
        };

        match format {
            FileExportFormat::Fbx => Self::Fbx(fbx::export::Options {
                storage: fbx::export::Storage::Binary,
                created: None,
            }),
            FileExportFormat::Glb => Self::Gltf(gltf::export::Options {
                storage: gltf::export::Storage::Binary,
                presentation: gltf::export::Presentation::Compact,
            }),
            FileExportFormat::Gltf => Self::Gltf(gltf::export::Options {
                storage: gltf::export::Storage::Embedded,
                presentation: gltf::export::Presentation::Pretty,
            }),
            FileExportFormat::Obj => Self::Obj(obj::export::Options {
                coords,
                units: src_unit,
            }),
            FileExportFormat::Ply => Self::Ply(ply::export::Options {
                storage: ply::export::Storage::Ascii,
                coords,
                selection: Selection::DefaultScene,
                units: src_unit,
            }),
            FileExportFormat::Step => Self::Step(step::export::Options { coords, created: None }),
            FileExportFormat::Stl => Self::Stl(stl::export::Options {
                storage: stl::export::Storage::Ascii,
                coords,
                units: src_unit,
                selection: Selection::DefaultScene,
            }),
        }
    }
}
