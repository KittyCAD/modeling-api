//! Types for sketching models.
use crate::{Destination, Instruction};
use kittycad_execution_plan_macros::ExecutionPlanValue;
use kittycad_execution_plan_traits::{Address, Value};
use kittycad_modeling_cmds::shared::{Point2d, Point3d, Point4d};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A sketch group is a collection of paths.
#[derive(Clone, ExecutionPlanValue, PartialEq, Debug, Deserialize, Serialize)]
pub struct SketchGroup {
    // NOTE to developers
    // Do NOT reorder these fields without updating the  _offset() methods below.
    /// The id of the sketch group.
    pub id: Uuid,
    /// What the sketch is on (can be a plane or a face).
    pub on: SketchSurface,
    /// The position of the sketch group.
    pub position: Point3d,
    /// The rotation of the sketch group base plane.
    pub rotation: Point4d,
    /// The X, Y and Z axes of this sketch's base plane, in 3D space.
    pub axes: Axes,
    /// The plane id or face id of the sketch group.
    pub entity_id: Option<Uuid>,
    /// The base path.
    pub path_first: BasePath,
    /// Paths after the first path, if any.
    pub path_rest: Vec<PathSegment>,
}

impl SketchGroup {
    /// Get the offset for the `id` field.
    pub fn path_id_offset() -> usize {
        0
    }
    /// Set the base path of the sketch group.
    /// `sketch_group` is the starting address of the sketch group.
    /// `start_point` is the address of the base path's `start` geometric point.
    /// `tag` is the address of the base path's `tag`.
    pub fn set_base_path(&self, sketch_group: Address, start_point: Address, tag: Option<Address>) -> Vec<Instruction> {
        let base_path_addr = sketch_group
            + self.id.into_parts().len()
            + self.on.into_parts().len()
            + self.position.into_parts().len()
            + self.rotation.into_parts().len()
            + self.axes.into_parts().len()
            + self.entity_id.into_parts().len()
            + self.entity_id.into_parts().len();
        let mut out = vec![
            // Copy over the `from` field.
            Instruction::Copy {
                source: start_point,
                destination: Destination::Address(base_path_addr),
                length: 1,
            },
            // Copy over the `to` field.
            Instruction::Copy {
                source: start_point,
                destination: Destination::Address(base_path_addr + self.path_first.from.into_parts().len()),
                length: 1,
            },
        ];
        if let Some(tag) = tag {
            // Copy over the `name` field.
            out.push(Instruction::Copy {
                source: tag,
                destination: Destination::Address(
                    base_path_addr + self.path_first.from.into_parts().len() + self.path_first.to.into_parts().len(),
                ),
                length: 1,
            });
        }
        out
    }
}

/// The X, Y and Z axes.
#[derive(Debug, Clone, Copy, ExecutionPlanValue, PartialEq, Deserialize, Serialize)]
pub struct Axes {
    #[allow(missing_docs)]
    pub x: Point3d,
    #[allow(missing_docs)]
    pub y: Point3d,
    #[allow(missing_docs)]
    pub z: Point3d,
}

/// A path which starts a SketchGroup.
#[derive(Debug, Clone, ExecutionPlanValue, PartialEq, Deserialize, Serialize)]
pub struct BasePath {
    /// Where the path starts.
    pub from: Point2d<f64>,
    /// Where the path ends.
    pub to: Point2d<f64>,
    /// The name of the path.
    pub name: String,
}

/// Paths are made up of multiple segments, laid out top-to-tail
/// (i.e. the end of one segment is the start of the next).
#[derive(Debug, Clone, ExecutionPlanValue, PartialEq, Deserialize, Serialize)]
pub enum PathSegment {
    /// A path that goes to a point.
    ToPoint {
        /// Defines the end point, and where the path segment to it started.
        base: BasePath,
    },
    /// A arc that is tangential to the last path segment that goes to a point
    TangentialArcTo {
        /// Defines the end point, and where the path segment to it started.
        base: BasePath,
        /// the arc's center
        center: Point2d,
        /// arc's direction
        ccw: bool,
    },
    /// A path that is horizontal.
    Horizontal {
        /// Defines the end point, and where the line to it started.
        base: BasePath,
        /// The x coordinate.
        x: f64,
    },
    /// An angled line to.
    AngledLineTo {
        /// Defines the end point, and where the line to it started.
        base: BasePath,
        /// The x coordinate.
        x: Option<f64>,
        /// The y coordinate.
        y: Option<f64>,
    },
    /// A base path.
    Base {
        /// Defines the end point, and where the line to it started.
        base: BasePath,
    },
}

impl PathSegment {
    /// What kind of segment?
    pub fn segment_kind(&self) -> &'static str {
        match self {
            PathSegment::ToPoint { .. } => "ToPoint",
            PathSegment::TangentialArcTo { .. } => "TangentialArcTo",
            PathSegment::Horizontal { .. } => "Horizontal",
            PathSegment::AngledLineTo { .. } => "AngledLineTo",
            PathSegment::Base { .. } => "Base",
        }
    }
}

/// What is being sketched on?
#[derive(Debug, Clone, Copy, ExecutionPlanValue, PartialEq, Deserialize, Serialize)]
pub enum SketchSurface {
    /// A plane.
    Plane(Plane),
}

/// A plane.
#[derive(Debug, Clone, Copy, ExecutionPlanValue, PartialEq, Deserialize, Serialize)]
pub struct Plane {
    /// The id of the plane.
    pub id: Uuid,
    /// The code for the plane either a string or custom.
    pub value: PlaneType,
    /// Origin of the plane.
    pub origin: Point3d,
    /// The plane's axes.
    pub axes: Axes,
}

/// Type for a plane.
#[derive(Debug, Clone, Copy, ExecutionPlanValue, PartialEq, Deserialize, Serialize)]
pub enum PlaneType {
    #[allow(missing_docs)]
    XY,
    #[allow(missing_docs)]
    XZ,
    #[allow(missing_docs)]
    YZ,
    /// A custom plane.
    Custom,
}
