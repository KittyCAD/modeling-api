use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::output;

/// A successful response from a modeling command.
/// This can be one of several types of responses, depending on the command.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "data")]
pub enum OkModelingCmdResponse {
    /// An empty response, used for any command that does not explicitly have a response
    /// defined here.
    Empty,
    /// The response from the `Export` command.
    /// When this is being performed over a websocket, this is sent as binary not JSON.
    /// The binary data can be deserialized as `bincode` into a `Vec<ExportFile>`.
    Export(output::Export),
    /// The response from the `SelectWithPoint` command.
    SelectWithPoint(output::SelectWithPoint),
    /// The response from the `HighlightSetEntity` command.
    HighlightSetEntity(output::HighlightSetEntity),
    /// The response from the `EntityGetChildUuid` command.
    EntityGetChildUuid(output::EntityGetChildUuid),
    /// The response from the `EntityGetNumChildren` command.
    EntityGetNumChildren(output::EntityGetNumChildren),
    /// The response from the `EntityGetParentId` command.
    EntityGetParentId(output::EntityGetParentId),
    /// The response from the `EntityGetAllChildUuids` command.
    EntityGetAllChildUuids(output::EntityGetAllChildUuids),
    /// The response from the `SelectGet` command.
    SelectGet(output::SelectGet),
    /// The response from the `GetEntityType` command.
    GetEntityType(output::GetEntityType),
    /// The response from the `Solid3dGetAllEdgeFaces` command.
    Solid3dGetAllEdgeFaces(output::Solid3dGetAllEdgeFaces),
    /// The response from the `Solid3dGetAllOppositeEdges` command.
    Solid3dGetAllOppositeEdges(output::Solid3dGetAllOppositeEdges),
    /// The response from the `Solid3dGetOppositeEdge` command.
    Solid3dGetOppositeEdge(output::Solid3dGetOppositeEdge),
    /// The response from the `Solid3dGetPrevAdjacentEdge` command.
    Solid3dGetPrevAdjacentEdge(output::Solid3dGetPrevAdjacentEdge),
    /// The response from the `Solid3dGetNextAdjacentEdge` command.
    Solid3dGetNextAdjacentEdge(output::Solid3dGetNextAdjacentEdge),
    /// The response from the `MouseClick` command.
    MouseClick(output::MouseClick),
    /// The response from the `CurveGetType` command.
    CurveGetType(output::CurveGetType),
    /// The response from the `CurveGetControlPoints` command.
    CurveGetControlPoints(output::CurveGetControlPoints),
    /// The response from the `Take Snapshot` command.
    TakeSnapshot(output::TakeSnapshot),
    /// The response from the `Path Get Info` command.
    PathGetInfo(output::PathGetInfo),
    /// The response from the `Path Get Curve UUIDs for Vertices` command.
    PathGetCurveUuidsForVertices(output::PathGetCurveUuidsForVertices),
    /// The response from the `Path Get Vertex UUIDs` command.
    PathGetVertexUuids(output::PathGetVertexUuids),
    /// The response from the `PlaneIntersectAndProject` command.
    PlaneIntersectAndProject(output::PlaneIntersectAndProject),
    /// The response from the `CurveGetEndPoints` command.
    CurveGetEndPoints(output::CurveGetEndPoints),
    /// The response from the `ImportFiles` command.
    ImportFiles(output::ImportFiles),
    /// The response from the `Mass` command.
    Mass(output::Mass),
    /// The response from the `Volume` command.
    Volume(output::Volume),
    /// The response from the `Density` command.
    Density(output::Density),
    /// The response from the `SurfaceArea` command.
    SurfaceArea(output::SurfaceArea),
    /// The response from the `CenterOfMass` command.
    CenterOfMass(output::CenterOfMass),
    /// The response from the `GetSketchModePlane` command.
    GetSketchModePlane(output::GetSketchModePlane),
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
