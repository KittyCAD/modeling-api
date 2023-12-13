use crate::{each_cmd::*, output as out, ModelingCmd, ModelingCmdVariant};

impl<'de> ModelingCmdVariant<'de> for MovePathPen {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::MovePathPen(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for ExtendPath {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::ExtendPath(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for Extrude {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::Extrude(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for ClosePath {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::ClosePath(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for CameraDragStart {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::CameraDragStart(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for CameraDragMove {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::CameraDragMove(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for CameraDragEnd {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::CameraDragEnd(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for EnableSketchMode {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::EnableSketchMode(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for DefaultCameraLookAt {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::DefaultCameraLookAt(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for DefaultCameraZoom {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::DefaultCameraZoom(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for DefaultCameraEnableSketchMode {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::DefaultCameraEnableSketchMode(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for DefaultCameraDisableSketchMode {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::DefaultCameraDisableSketchMode(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for DefaultCameraFocusOn {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::DefaultCameraFocusOn(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for Export {
    type Output = out::Export;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::Export(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for EntityGetParentId {
    type Output = out::EntityGetParentId;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::EntityGetParentId(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for EntityGetNumChildren {
    type Output = out::EntityGetNumChildren;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::EntityGetNumChildren(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for EntityGetChildUuid {
    type Output = out::EntityGetChildUuid;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::EntityGetChildUuid(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for EntityGetAllChildUuids {
    type Output = out::EntityGetAllChildUuids;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::EntityGetAllChildUuids(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for EntityGetDistance {
    type Output = out::EntityGetDistance;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::EntityGetDistance(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for EditModeEnter {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::EditModeEnter(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for SelectWithPoint {
    type Output = out::SelectWithPoint;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::SelectWithPoint(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for SelectAdd {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::SelectAdd(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for SelectRemove {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::SelectRemove(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for SelectReplace {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::SelectReplace(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for HighlightSetEntity {
    type Output = out::HighlightSetEntity;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::HighlightSetEntity(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for HighlightSetEntities {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::HighlightSetEntities(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for NewAnnotation {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::NewAnnotation(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for UpdateAnnotation {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::UpdateAnnotation(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for ObjectVisible {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::ObjectVisible(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for ObjectBringToFront {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::ObjectBringToFront(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for GetEntityType {
    type Output = out::GetEntityType;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::GetEntityType(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for Solid2dAddHole {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::Solid2dAddHole(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for Solid3dGetAllEdgeFaces {
    type Output = out::Solid3dGetAllEdgeFaces;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::Solid3dGetAllEdgeFaces(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for Solid3dGetAllOppositeEdges {
    type Output = out::Solid3dGetAllOppositeEdges;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::Solid3dGetAllOppositeEdges(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for Solid3dGetOppositeEdge {
    type Output = out::Solid3dGetOppositeEdge;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::Solid3dGetOppositeEdge(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for Solid3dGetNextAdjacentEdge {
    type Output = out::Solid3dGetNextAdjacentEdge;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::Solid3dGetNextAdjacentEdge(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for Solid3dGetPrevAdjacentEdge {
    type Output = out::Solid3dGetPrevAdjacentEdge;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::Solid3dGetPrevAdjacentEdge(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for SendObject {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::SendObject(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for ObjectSetMaterialParamsPbr {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::ObjectSetMaterialParamsPbr(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for EntitySetOpacity {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::EntitySetOpacity(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for EntityFade {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::EntityFade(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for MakePlane {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::MakePlane(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for PlaneSetColor {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::PlaneSetColor(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for SetTool {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::SetTool(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for MouseMove {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::MouseMove(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for MouseClick {
    type Output = out::MouseClick;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::MouseClick(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for SketchModeEnable {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::SketchModeEnable(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for SketchModeDisable {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::SketchModeDisable(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for CurveGetType {
    type Output = out::CurveGetType;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::CurveGetType(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for CurveGetControlPoints {
    type Output = out::CurveGetControlPoints;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::CurveGetControlPoints(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for TakeSnapshot {
    type Output = out::TakeSnapshot;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::TakeSnapshot(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for MakeAxesGizmo {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::MakeAxesGizmo(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for PathGetInfo {
    type Output = out::PathGetInfo;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::PathGetInfo(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for PathGetCurveUuidsForVertices {
    type Output = out::PathGetCurveUuidsForVertices;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::PathGetCurveUuidsForVertices(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for PathGetVertexUuids {
    type Output = out::PathGetVertexUuids;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::PathGetVertexUuids(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for HandleMouseDragStart {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::HandleMouseDragStart(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for HandleMouseDragMove {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::HandleMouseDragMove(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for HandleMouseDragEnd {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::HandleMouseDragEnd(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for RemoveSceneObjects {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::RemoveSceneObjects(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for CurveSetConstraint {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::CurveSetConstraint(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for PlaneIntersectAndProject {
    type Output = out::PlaneIntersectAndProject;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::PlaneIntersectAndProject(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for CurveGetEndPoints {
    type Output = out::CurveGetEndPoints;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::CurveGetEndPoints(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for ReconfigureStream {
    type Output = ();
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::ReconfigureStream(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for ImportFiles {
    type Output = out::ImportFiles;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::ImportFiles(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for Mass {
    type Output = out::Mass;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::Mass(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for Volume {
    type Output = out::Volume;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::Volume(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for Density {
    type Output = out::Density;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::Density(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for SurfaceArea {
    type Output = out::SurfaceArea;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::SurfaceArea(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for CenterOfMass {
    type Output = out::CenterOfMass;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::CenterOfMass(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for GetSketchModePlane {
    type Output = out::GetSketchModePlane;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::GetSketchModePlane(self)
    }
}
impl<'de> ModelingCmdVariant<'de> for EntityLinearPattern {
    type Output = out::GetSketchModePlane;
    fn into_enum(self) -> ModelingCmd {
        ModelingCmd::EntityLinearPattern(self)
    }
}
