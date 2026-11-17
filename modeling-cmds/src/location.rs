//! Types of identifying a specific location on a body, face or curve/edge.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::WhichEdge;

/// Defines a unique location on curve geometry.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "Location.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub enum CurveLocation {
    /// Location on the parametric representation of the curve.
    Param {
        /// The parametric value.
        /// 0 is the start, 1 is the end, 0.5 is the midpoint.
        t: f64,
    },
    /// The center of a circular edge.
    /// Works with circles or circular arcs.
    CircularCenter {},
    // Quick followup after MVP:
    // Centroid,
    // Later followup:
    // Mirrored from center of circle across circular arc
}

/// Defines a unique location on surface geometry.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "Location.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub enum SurfaceLocation {
    /// Location on the parametric representation of the surface.
    Param {
        /// First parametric value
        u: f64,
        /// Second parametric value
        v: f64,
    },
    // Quick followup after MVP:
    // Centroid,
}

/// Somewhere specific on a 3D body.
/// Could be used to create an anchor, and potentially other purposes in the future.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "Location.ts"))]
#[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
pub enum BodyLocation {
    /// A specific location on a specific face.
    Face {
        /// A specific face.
        face_id: Uuid,
        /// A specific location on the face.
        location: SurfaceLocation,
    },

    /// A specific location on a specific edge.
    /// Useful for getting at a vertex.
    Edge {
        /// A specific edge.
        which_edge: WhichEdge,
        /// A specific location on the edge.
        location: CurveLocation,
    },
    // Quick followup after MVP:
    // CopyAnchor
    // Later followup:
    // SketchPoint, (hard currently because sketch points aren't sent to the engine, and would need to be mapped from 2D to 3D space)
}
