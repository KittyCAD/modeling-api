use crate::{each_cmd::*, output as out, ModelingCmd, ModelingCmdVariant};

impl<'de> ModelingCmdVariant<'de> for StartPath {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::StartPath(self)
    }
    fn name() -> &'static str {
        "StartPath"
    }
}
impl<'de> ModelingCmdVariant<'de> for MovePathPen {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::MovePathPen(self)
    }
    fn name() -> &'static str {
        "MovePathPen"
    }
}
impl<'de> ModelingCmdVariant<'de> for ExtendPath {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::ExtendPath(self)
    }
    fn name() -> &'static str {
        "ExtendPath"
    }
}
impl<'de> ModelingCmdVariant<'de> for Extrude {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::Extrude(self)
    }
    fn name() -> &'static str {
        "Extrude"
    }
}
impl<'de> ModelingCmdVariant<'de> for ClosePath {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::ClosePath(self)
    }
    fn name() -> &'static str {
        "ClosePath"
    }
}
impl<'de> ModelingCmdVariant<'de> for CameraDragStart {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::CameraDragStart(self)
    }
    fn name() -> &'static str {
        "CameraDragStart"
    }
}
impl<'de> ModelingCmdVariant<'de> for CameraDragMove {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::CameraDragMove(self)
    }
    fn name() -> &'static str {
        "CameraDragMove"
    }
}
impl<'de> ModelingCmdVariant<'de> for CameraDragEnd {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::CameraDragEnd(self)
    }
    fn name() -> &'static str {
        "CameraDragEnd"
    }
}
impl<'de> ModelingCmdVariant<'de> for EnableSketchMode {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::EnableSketchMode(self)
    }
    fn name() -> &'static str {
        "EnableSketchMode"
    }
}
impl<'de> ModelingCmdVariant<'de> for DefaultCameraLookAt {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::DefaultCameraLookAt(self)
    }
    fn name() -> &'static str {
        "DefaultCameraLookAt"
    }
}
impl<'de> ModelingCmdVariant<'de> for DefaultCameraZoom {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::DefaultCameraZoom(self)
    }
    fn name() -> &'static str {
        "DefaultCameraZoom"
    }
}
impl<'de> ModelingCmdVariant<'de> for DefaultCameraEnableSketchMode {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::DefaultCameraEnableSketchMode(self)
    }
    fn name() -> &'static str {
        "DefaultCameraEnableSketchMode"
    }
}
impl<'de> ModelingCmdVariant<'de> for DefaultCameraDisableSketchMode {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::DefaultCameraDisableSketchMode(self)
    }
    fn name() -> &'static str {
        "DefaultCameraDisableSketchMode"
    }
}
impl<'de> ModelingCmdVariant<'de> for DefaultCameraFocusOn {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::DefaultCameraFocusOn(self)
    }
    fn name() -> &'static str {
        "DefaultCameraFocusOn"
    }
}
impl<'de> ModelingCmdVariant<'de> for Export {
    type Output = out::Export;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::Export(self)
    }
    fn name() -> &'static str {
        "Export"
    }
}
impl<'de> ModelingCmdVariant<'de> for EntityGetParentId {
    type Output = out::EntityGetParentId;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::EntityGetParentId(self)
    }
    fn name() -> &'static str {
        "EntityGetParentId"
    }
}
impl<'de> ModelingCmdVariant<'de> for EntityGetNumChildren {
    type Output = out::EntityGetNumChildren;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::EntityGetNumChildren(self)
    }
    fn name() -> &'static str {
        "EntityGetNumChildren"
    }
}
impl<'de> ModelingCmdVariant<'de> for EntityGetChildUuid {
    type Output = out::EntityGetChildUuid;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::EntityGetChildUuid(self)
    }
    fn name() -> &'static str {
        "EntityGetChildUuid"
    }
}
impl<'de> ModelingCmdVariant<'de> for EntityGetAllChildUuids {
    type Output = out::EntityGetAllChildUuids;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::EntityGetAllChildUuids(self)
    }
    fn name() -> &'static str {
        "EntityGetAllChildUuids"
    }
}
impl<'de> ModelingCmdVariant<'de> for EntityGetDistance {
    type Output = out::EntityGetDistance;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::EntityGetDistance(self)
    }
    fn name() -> &'static str {
        "EntityGetDistance"
    }
}
impl<'de> ModelingCmdVariant<'de> for EditModeEnter {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::EditModeEnter(self)
    }
    fn name() -> &'static str {
        "EditModeEnter"
    }
}
impl<'de> ModelingCmdVariant<'de> for SelectWithPoint {
    type Output = out::SelectWithPoint;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::SelectWithPoint(self)
    }
    fn name() -> &'static str {
        "SelectWithPoint"
    }
}
impl<'de> ModelingCmdVariant<'de> for SelectAdd {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::SelectAdd(self)
    }
    fn name() -> &'static str {
        "SelectAdd"
    }
}
impl<'de> ModelingCmdVariant<'de> for SelectRemove {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::SelectRemove(self)
    }
    fn name() -> &'static str {
        "SelectRemove"
    }
}
impl<'de> ModelingCmdVariant<'de> for SelectReplace {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::SelectReplace(self)
    }
    fn name() -> &'static str {
        "SelectReplace"
    }
}
impl<'de> ModelingCmdVariant<'de> for HighlightSetEntity {
    type Output = out::HighlightSetEntity;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::HighlightSetEntity(self)
    }
    fn name() -> &'static str {
        "HighlightSetEntity"
    }
}
impl<'de> ModelingCmdVariant<'de> for HighlightSetEntities {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::HighlightSetEntities(self)
    }
    fn name() -> &'static str {
        "HighlightSetEntities"
    }
}
impl<'de> ModelingCmdVariant<'de> for NewAnnotation {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::NewAnnotation(self)
    }
    fn name() -> &'static str {
        "NewAnnotation"
    }
}
impl<'de> ModelingCmdVariant<'de> for UpdateAnnotation {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::UpdateAnnotation(self)
    }
    fn name() -> &'static str {
        "UpdateAnnotation"
    }
}
impl<'de> ModelingCmdVariant<'de> for ObjectVisible {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::ObjectVisible(self)
    }
    fn name() -> &'static str {
        "ObjectVisible"
    }
}
impl<'de> ModelingCmdVariant<'de> for ObjectBringToFront {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::ObjectBringToFront(self)
    }
    fn name() -> &'static str {
        "ObjectBringToFront"
    }
}
impl<'de> ModelingCmdVariant<'de> for GetEntityType {
    type Output = out::GetEntityType;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::GetEntityType(self)
    }
    fn name() -> &'static str {
        "GetEntityType"
    }
}
impl<'de> ModelingCmdVariant<'de> for Solid2dAddHole {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::Solid2dAddHole(self)
    }
    fn name() -> &'static str {
        "Solid2dAddHole"
    }
}
impl<'de> ModelingCmdVariant<'de> for Solid3dGetAllEdgeFaces {
    type Output = out::Solid3dGetAllEdgeFaces;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::Solid3dGetAllEdgeFaces(self)
    }
    fn name() -> &'static str {
        "Solid3dGetAllEdgeFaces"
    }
}
impl<'de> ModelingCmdVariant<'de> for Solid3dGetAllOppositeEdges {
    type Output = out::Solid3dGetAllOppositeEdges;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::Solid3dGetAllOppositeEdges(self)
    }
    fn name() -> &'static str {
        "Solid3dGetAllOppositeEdges"
    }
}
impl<'de> ModelingCmdVariant<'de> for Solid3dGetOppositeEdge {
    type Output = out::Solid3dGetOppositeEdge;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::Solid3dGetOppositeEdge(self)
    }
    fn name() -> &'static str {
        "Solid3dGetOppositeEdge"
    }
}
impl<'de> ModelingCmdVariant<'de> for Solid3dGetNextAdjacentEdge {
    type Output = out::Solid3dGetNextAdjacentEdge;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::Solid3dGetNextAdjacentEdge(self)
    }
    fn name() -> &'static str {
        "Solid3dGetNextAdjacentEdge"
    }
}
impl<'de> ModelingCmdVariant<'de> for Solid3dGetPrevAdjacentEdge {
    type Output = out::Solid3dGetPrevAdjacentEdge;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::Solid3dGetPrevAdjacentEdge(self)
    }
    fn name() -> &'static str {
        "Solid3dGetPrevAdjacentEdge"
    }
}
impl<'de> ModelingCmdVariant<'de> for SendObject {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::SendObject(self)
    }
    fn name() -> &'static str {
        "SendObject"
    }
}
impl<'de> ModelingCmdVariant<'de> for ObjectSetMaterialParamsPbr {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::ObjectSetMaterialParamsPbr(self)
    }
    fn name() -> &'static str {
        "ObjectSetMaterialParamsPbr"
    }
}
impl<'de> ModelingCmdVariant<'de> for EntitySetOpacity {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::EntitySetOpacity(self)
    }
    fn name() -> &'static str {
        "EntitySetOpacity"
    }
}
impl<'de> ModelingCmdVariant<'de> for EntityFade {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::EntityFade(self)
    }
    fn name() -> &'static str {
        "EntityFade"
    }
}
impl<'de> ModelingCmdVariant<'de> for MakePlane {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::MakePlane(self)
    }
    fn name() -> &'static str {
        "MakePlane"
    }
}
impl<'de> ModelingCmdVariant<'de> for PlaneSetColor {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::PlaneSetColor(self)
    }
    fn name() -> &'static str {
        "PlaneSetColor"
    }
}
impl<'de> ModelingCmdVariant<'de> for SetTool {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::SetTool(self)
    }
    fn name() -> &'static str {
        "SetTool"
    }
}
impl<'de> ModelingCmdVariant<'de> for MouseMove {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::MouseMove(self)
    }
    fn name() -> &'static str {
        "MouseMove"
    }
}
impl<'de> ModelingCmdVariant<'de> for MouseClick {
    type Output = out::MouseClick;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::MouseClick(self)
    }
    fn name() -> &'static str {
        "MouseClick"
    }
}
impl<'de> ModelingCmdVariant<'de> for SketchModeEnable {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::SketchModeEnable(self)
    }
    fn name() -> &'static str {
        "SketchModeEnable"
    }
}
impl<'de> ModelingCmdVariant<'de> for SketchModeDisable {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::SketchModeDisable(self)
    }
    fn name() -> &'static str {
        "SketchModeDisable"
    }
}
impl<'de> ModelingCmdVariant<'de> for CurveGetType {
    type Output = out::CurveGetType;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::CurveGetType(self)
    }
    fn name() -> &'static str {
        "CurveGetType"
    }
}
impl<'de> ModelingCmdVariant<'de> for CurveGetControlPoints {
    type Output = out::CurveGetControlPoints;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::CurveGetControlPoints(self)
    }
    fn name() -> &'static str {
        "CurveGetControlPoints"
    }
}
impl<'de> ModelingCmdVariant<'de> for TakeSnapshot {
    type Output = out::TakeSnapshot;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::TakeSnapshot(self)
    }
    fn name() -> &'static str {
        "TakeSnapshot"
    }
}
impl<'de> ModelingCmdVariant<'de> for MakeAxesGizmo {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::MakeAxesGizmo(self)
    }
    fn name() -> &'static str {
        "MakeAxesGizmo"
    }
}
impl<'de> ModelingCmdVariant<'de> for PathGetInfo {
    type Output = out::PathGetInfo;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::PathGetInfo(self)
    }
    fn name() -> &'static str {
        "PathGetInfo"
    }
}
impl<'de> ModelingCmdVariant<'de> for PathGetCurveUuidsForVertices {
    type Output = out::PathGetCurveUuidsForVertices;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::PathGetCurveUuidsForVertices(self)
    }
    fn name() -> &'static str {
        "PathGetCurveUuidsForVertices"
    }
}
impl<'de> ModelingCmdVariant<'de> for PathGetVertexUuids {
    type Output = out::PathGetVertexUuids;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::PathGetVertexUuids(self)
    }
    fn name() -> &'static str {
        "PathGetVertexUuids"
    }
}
impl<'de> ModelingCmdVariant<'de> for HandleMouseDragStart {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::HandleMouseDragStart(self)
    }
    fn name() -> &'static str {
        "HandleMouseDragStart"
    }
}
impl<'de> ModelingCmdVariant<'de> for HandleMouseDragMove {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::HandleMouseDragMove(self)
    }
    fn name() -> &'static str {
        "HandleMouseDragMove"
    }
}
impl<'de> ModelingCmdVariant<'de> for HandleMouseDragEnd {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::HandleMouseDragEnd(self)
    }
    fn name() -> &'static str {
        "HandleMouseDragEnd"
    }
}
impl<'de> ModelingCmdVariant<'de> for RemoveSceneObjects {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::RemoveSceneObjects(self)
    }
    fn name() -> &'static str {
        "RemoveSceneObjects"
    }
}
impl<'de> ModelingCmdVariant<'de> for CurveSetConstraint {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::CurveSetConstraint(self)
    }
    fn name() -> &'static str {
        "CurveSetConstraint"
    }
}
impl<'de> ModelingCmdVariant<'de> for PlaneIntersectAndProject {
    type Output = out::PlaneIntersectAndProject;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::PlaneIntersectAndProject(self)
    }
    fn name() -> &'static str {
        "PlaneIntersectAndProject"
    }
}
impl<'de> ModelingCmdVariant<'de> for CurveGetEndPoints {
    type Output = out::CurveGetEndPoints;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::CurveGetEndPoints(self)
    }
    fn name() -> &'static str {
        "CurveGetEndPoints"
    }
}
impl<'de> ModelingCmdVariant<'de> for ReconfigureStream {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::ReconfigureStream(self)
    }
    fn name() -> &'static str {
        "ReconfigureStream"
    }
}
impl<'de> ModelingCmdVariant<'de> for ImportFiles {
    type Output = out::ImportFiles;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::ImportFiles(self)
    }
    fn name() -> &'static str {
        "ImportFiles"
    }
}
impl<'de> ModelingCmdVariant<'de> for Mass {
    type Output = out::Mass;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::Mass(self)
    }
    fn name() -> &'static str {
        "Mass"
    }
}
impl<'de> ModelingCmdVariant<'de> for Volume {
    type Output = out::Volume;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::Volume(self)
    }
    fn name() -> &'static str {
        "Volume"
    }
}
impl<'de> ModelingCmdVariant<'de> for Density {
    type Output = out::Density;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::Density(self)
    }
    fn name() -> &'static str {
        "Density"
    }
}
impl<'de> ModelingCmdVariant<'de> for SurfaceArea {
    type Output = out::SurfaceArea;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::SurfaceArea(self)
    }
    fn name() -> &'static str {
        "SurfaceArea"
    }
}
impl<'de> ModelingCmdVariant<'de> for CenterOfMass {
    type Output = out::CenterOfMass;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::CenterOfMass(self)
    }
    fn name() -> &'static str {
        "CenterOfMass"
    }
}
impl<'de> ModelingCmdVariant<'de> for GetSketchModePlane {
    type Output = out::GetSketchModePlane;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::GetSketchModePlane(self)
    }
    fn name() -> &'static str {
        "GetSketchModePlane"
    }
}
