use crate::{each_cmd::*, output as out, ModelingCmdVariant};

impl<'de> ModelingCmdVariant<'de> for MovePathPen {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for ExtendPath {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for Extrude {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for ClosePath {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for CameraDragStart {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for CameraDragMove {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for CameraDragEnd {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for EnableSketchMode {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for DefaultCameraLookAt {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for DefaultCameraZoom {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for DefaultCameraEnableSketchMode {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for DefaultCameraDisableSketchMode {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for DefaultCameraFocusOn {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for Export {
    type Output = out::Export;
}

impl<'de> ModelingCmdVariant<'de> for EntityGetParentId {
    type Output = out::EntityGetParentId;
}

impl<'de> ModelingCmdVariant<'de> for EntityGetNumChildren {
    type Output = out::EntityGetNumChildren;
}

impl<'de> ModelingCmdVariant<'de> for EntityGetChildUuid {
    type Output = out::EntityGetChildUuid;
}

impl<'de> ModelingCmdVariant<'de> for EntityGetAllChildUuids {
    type Output = out::EntityGetAllChildUuids;
}

impl<'de> ModelingCmdVariant<'de> for EntityGetDistance {
    type Output = out::EntityGetDistance;
}

impl<'de> ModelingCmdVariant<'de> for EditModeEnter {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for SelectWithPoint {
    type Output = out::SelectWithPoint;
}

impl<'de> ModelingCmdVariant<'de> for SelectAdd {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for SelectRemove {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for SelectReplace {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for HighlightSetEntity {
    type Output = out::HighlightSetEntity;
}

impl<'de> ModelingCmdVariant<'de> for HighlightSetEntities {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for NewAnnotation {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for UpdateAnnotation {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for ObjectVisible {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for ObjectBringToFront {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for GetEntityType {
    type Output = out::GetEntityType;
}
impl<'de> ModelingCmdVariant<'de> for Solid2dAddHole {
    type Output = ();
}
impl<'de> ModelingCmdVariant<'de> for Solid3dGetAllEdgeFaces {
    type Output = out::Solid3dGetAllEdgeFaces;
}
impl<'de> ModelingCmdVariant<'de> for Solid3dGetAllOppositeEdges {
    type Output = out::Solid3dGetAllOppositeEdges;
}
impl<'de> ModelingCmdVariant<'de> for Solid3dGetOppositeEdge {
    type Output = out::Solid3dGetOppositeEdge;
}
impl<'de> ModelingCmdVariant<'de> for Solid3dGetNextAdjacentEdge {
    type Output = out::Solid3dGetNextAdjacentEdge;
}
impl<'de> ModelingCmdVariant<'de> for Solid3dGetPrevAdjacentEdge {
    type Output = out::Solid3dGetPrevAdjacentEdge;
}
impl<'de> ModelingCmdVariant<'de> for SendObject {
    type Output = ();
}
impl<'de> ModelingCmdVariant<'de> for ObjectSetMaterialParamsPBR {
    type Output = ();
}
impl<'de> ModelingCmdVariant<'de> for EntitySetOpacity {
    type Output = ();
}
impl<'de> ModelingCmdVariant<'de> for EntityFade {
    type Output = ();
}
impl<'de> ModelingCmdVariant<'de> for MakePlane {
    type Output = ();
}
impl<'de> ModelingCmdVariant<'de> for PlaneSetColor {
    type Output = ();
}
impl<'de> ModelingCmdVariant<'de> for SetTool {
    type Output = ();
}
impl<'de> ModelingCmdVariant<'de> for MouseMove {
    type Output = ();
}
impl<'de> ModelingCmdVariant<'de> for MouseClick {
    type Output = out::MouseClick;
}
impl<'de> ModelingCmdVariant<'de> for SketchModeEnable {
    type Output = ();
}
impl<'de> ModelingCmdVariant<'de> for SketchModeDisable {
    type Output = ();
}
impl<'de> ModelingCmdVariant<'de> for CurveGetType {
    type Output = out::CurveGetType;
}
impl<'de> ModelingCmdVariant<'de> for CurveGetControlPoints {
    type Output = out::CurveGetControlPoints;
}
impl<'de> ModelingCmdVariant<'de> for TakeSnapshot {
    type Output = out::TakeSnapshot;
}
impl<'de> ModelingCmdVariant<'de> for MakeAxesGizmo {
    type Output = ();
}
impl<'de> ModelingCmdVariant<'de> for PathGetInfo {
    type Output = out::PathGetInfo;
}
impl<'de> ModelingCmdVariant<'de> for PathGetCurveUuidsForVertices {
    type Output = out::PathGetCurveUuidsForVertices;
}
impl<'de> ModelingCmdVariant<'de> for PathGetVertexUuids {
    type Output = out::PathGetVertexUuids;
}
impl<'de> ModelingCmdVariant<'de> for HandleMouseDragStart {
    type Output = ();
}
impl<'de> ModelingCmdVariant<'de> for HandleMouseDragMove {
    type Output = ();
}
impl<'de> ModelingCmdVariant<'de> for HandleMouseDragEnd {
    type Output = ();
}
impl<'de> ModelingCmdVariant<'de> for RemoveSceneObjects {
    type Output = ();
}
impl<'de> ModelingCmdVariant<'de> for CurveSetConstraint {
    type Output = ();
}

impl<'de> ModelingCmdVariant<'de> for PlaneIntersectAndProject {
    type Output = out::PlaneIntersectAndProject;
}
impl<'de> ModelingCmdVariant<'de> for CurveGetEndPoints {
    type Output = out::CurveGetEndPoints;
}
impl<'de> ModelingCmdVariant<'de> for ReconfigureStream {
    type Output = ();
}
impl<'de> ModelingCmdVariant<'de> for ImportFiles {
    type Output = out::ImportFiles;
}
impl<'de> ModelingCmdVariant<'de> for Mass {
    type Output = out::Mass;
}
impl<'de> ModelingCmdVariant<'de> for Volume {
    type Output = out::Volume;
}
impl<'de> ModelingCmdVariant<'de> for Density {
    type Output = out::Density;
}
impl<'de> ModelingCmdVariant<'de> for SurfaceArea {
    type Output = out::SurfaceArea;
}
impl<'de> ModelingCmdVariant<'de> for CenterOfMass {
    type Output = out::CenterOfMass;
}
impl<'de> ModelingCmdVariant<'de> for GetSketchModePlane {
    type Output = out::GetSketchModePlane;
}
