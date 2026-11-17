//! Data types for assemblies and mates.
use kittycad_point::Point3d;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    length_unit::LengthUnit,
    shared::{EdgeSpecifier, Rotation},
};

/// What to create an anchor on
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub enum AnchorOn {
    /// Mate frame at the centroid of this face.
    Face {
        /// Which face?
        which_face: Uuid,
        /// Where on the given face?
        where_on_face: WhereOnFace,
    },
    /// Mate frame along this edge.
    Edge {
        /// Which edge?
        which_edge: EdgeSpecifier,
        /// Where on the given edge?
        where_on_edge: WhereOnEdge,
    },
}

/// Identifies a point on a specific face.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub enum WhereOnFace {
    /// Centermost point of the face.
    Centroid {},
    /// Somewhere along the face.
    AtParameter {
        /// Value for the face's 2D parametric equation.
        #[schemars(range(min = 0.0, max = 1.0))]
        u: f64,
        /// Value for the face's 2D parametric equation.
        #[schemars(range(min = 0.0, max = 1.0))]
        v: f64,
    },
}

/// Identifies a point on a specific edge.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub enum WhereOnEdge {
    /// Midpoint along the edge.
    Midpoint {},
    /// Somewhere along the edge's length.
    AtParameter {
        /// Value which, substituted into the edge's parametric equation,
        /// gives a point. Must be between 0 and 1 (inclusive).
        #[schemars(range(min = 0.0, max = 1.0))]
        t: f64,
    },
}

/// How to two bodies in an assembly.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "Mate.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub struct Mate {
    /// Which two anchors are being constrained
    pub anchor_ids: [Uuid; 2],

    /// How to constrain the two anchors.
    pub data: MateData,
}

/// How to constrain bodies in an assembly.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "Mate.ts"))]
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub struct SolvedAssemblyComponent {
    /// How to translate the item.
    /// Applied before rotation.
    pub translate_by: Point3d<LengthUnit>,
    /// How to rotate the item.
    /// Applied after rotation.
    pub rotate_by: Rotation,
}
