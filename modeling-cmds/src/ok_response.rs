use kittycad_execution_plan_macros::ExecutionPlanValue;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::output;

macro_rules! build_enum {
    ($( $variant:ident ),* ) => {
/// A successful response from a modeling command.
/// This can be one of several types of responses, depending on the command.
#[derive(Debug, Serialize, Deserialize, JsonSchema, ExecutionPlanValue)]
#[serde(rename_all = "snake_case", tag = "type", content = "data")]
pub enum OkModelingCmdResponse {
    /// An empty response, used for any command that does not explicitly have a response
    /// defined here.
    Empty,
    $(
        #[doc = "The response from the `"]
        #[doc = stringify!($variant)]
        #[doc = "` command."]
        $variant(output::$variant),
    )* }
    };
}

build_enum! {
    Export,
    SelectWithPoint,
    HighlightSetEntity,
    EntityGetChildUuid,
    EntityGetNumChildren,
    EntityGetParentId,
    EntityGetAllChildUuids,
    SelectGet,
    GetEntityType,
    EntityGetDistance,
    EntityLinearPattern,
    Solid3dGetAllEdgeFaces,
    Solid3dGetAllOppositeEdges,
    Solid3dGetOppositeEdge,
    Solid3dGetPrevAdjacentEdge,
    Solid3dGetNextAdjacentEdge,
    MouseClick,
    CurveGetType,
    CurveGetControlPoints,
    TakeSnapshot,
    PathGetInfo,
    PathGetCurveUuidsForVertices,
    PathGetVertexUuids,
    PlaneIntersectAndProject,
    CurveGetEndPoints,
    ImportFiles,
    Mass,
    Volume,
    Density,
    SurfaceArea,
    CenterOfMass,
    GetSketchModePlane
}

impl From<output::ImportFiles> for OkModelingCmdResponse {
    fn from(x: output::ImportFiles) -> Self {
        Self::ImportFiles(x)
    }
}

impl From<output::CurveGetEndPoints> for OkModelingCmdResponse {
    fn from(x: output::CurveGetEndPoints) -> Self {
        Self::CurveGetEndPoints(x)
    }
}

impl From<output::PlaneIntersectAndProject> for OkModelingCmdResponse {
    fn from(x: output::PlaneIntersectAndProject) -> Self {
        Self::PlaneIntersectAndProject(x)
    }
}

impl From<()> for OkModelingCmdResponse {
    fn from(_: ()) -> Self {
        Self::Empty
    }
}
