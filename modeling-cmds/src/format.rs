use parse_display_derive::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod fbx;
pub mod gltf;
pub mod obj;
pub mod ply;
pub mod step;
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
    /// If you prefer ascii output, you can set that option for the export.
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
