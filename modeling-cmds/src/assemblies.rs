//! Data types for assemblies and mates.
use bon::Builder;
use kittycad_point::Point3d;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{length_unit::LengthUnit, location::BodyLocation, shared::Rotation};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "Assemblies.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub enum AnchorOrientation {
    /// The face might not be planar.
    /// If not planar, samples face normal at the closest point on face to AnchorLocation.
    FaceNormal { entity_id: Uuid },

    /// The edge might not be straight.
    /// If not straight, samples edge normal at the closest point on edge to AnchorLocation.
    EdgeDirection { which_edge: crate::shared::WhichEdge },
    // After MVP add variants like:
    // Sketch normal
    // CopyFromAnchor
    // Between {a: Anchor, b: Anchor, weight: f64 = 0.5}
}

/// How to two bodies in an assembly.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema, Builder)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "Assemblies.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub struct AnchorCreateParams {
    /// Where to place the anchor
    pub location: BodyLocation,

    /// How to orient the anchor
    pub orientation: AnchorOrientation,

    /// These will get applied in order.
    #[serde(default)]
    #[builder(default)]
    pub transforms: Vec<AnchorTransform>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "Assemblies.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub enum AnchorTransform {
    /// Flip the Z axis, from positive to negative.
    FlipZ {},

    /// Rotate the other two axes about X.
    RotateAboutX { degrees: f64 },

    /// Rotate the other two axes about Y.
    RotateAboutY { degrees: f64 },

    /// Rotate the other two axes about Z.
    RotateAboutZ { degrees: f64 },

    /// Translate the anchor in its local coordinate system.
    Translate { vector: Point3d<LengthUnit> },
}

/// How to two bodies in an assembly.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema, Builder)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "Assemblies.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub struct Mate {
    /// Which two anchors are being constrained
    pub targets: [MateTarget; 2],

    /// How to constrain the two anchors.
    pub data: MateData,
}

/// How to two bodies in an assembly.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema, Builder)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "Assemblies.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub struct MateTarget {
    /// Which part is being mated?
    pub part_instance_id: Uuid,

    /// Which anchor on that part is being mated?
    pub anchor_id: Uuid,
}

/// How to constrain bodies in an assembly.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "Assemblies.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub enum MateData {
    /// Ball
    Spherical {},

    /// The two anchors must be this Euclidean distance apart.
    Distance {
        /// The desired distance.
        distance: LengthUnit,
    },

    /// Hinge
    Revolute {},

    /// Constrain the bodies to a single axis,
    /// while allowing them to rotate about and slide along that axis.
    Cylindrical {},

    /// Fixed, fastened
    Weld {},

    /// Slider
    Prismatic {},

    /// Hooke
    Universal {},

    /// The normals of these planes must be parallel.
    /// The planes can still slide in X and Y directions,
    /// or rotate around their normal.
    /// 3 degrees of freedom.
    Planar {},

    /// One chosen point on the second body stays in a plane attached to the first.
    /// That point can slide within the plane, and the body can rotate freely around that point.
    /// 5 degrees of freedom.
    PointOnPlane {},
}

/// Result of constraining an item in an assembly.
/// This is in the coordinate system of the assembly's parent.
/// Translation should be applied before rotation.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema, Builder)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "Assemblies.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub struct SolvedAssemblyComponent {
    /// How to translate the item.
    /// Applied before rotation.
    pub translate_by: Point3d<LengthUnit>,
    /// How to rotate the item.
    /// Applied after rotation.
    pub rotate_by: Rotation,
}
