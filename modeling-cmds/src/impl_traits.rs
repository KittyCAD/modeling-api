use crate::{each_cmd::*, output as out, ModelingCmd, ModelingCmdVariant};

macro_rules! impl_variant_output {
    ($struct:ident) => {
        impl ModelingCmdVariant for $struct {
            type Output = out::$struct;
            fn into_enum(self) -> ModelingCmd {
                ModelingCmd::$struct(self)
            }
            fn name() -> &'static str {
                stringify!($struct)
            }
        }
    };
}

macro_rules! impl_variant_empty {
    ($struct:ident) => {
        impl ModelingCmdVariant for $struct {
            type Output = ();
            fn into_enum(self) -> ModelingCmd {
                ModelingCmd::$struct(self)
            }
            fn name() -> &'static str {
                stringify!($struct)
            }
        }
    };
}

impl_variant_empty!(StartPath);
impl_variant_empty!(MovePathPen);
impl_variant_empty!(ExtendPath);
impl_variant_empty!(Extrude);
impl_variant_empty!(ClosePath);
impl_variant_empty!(CameraDragStart);
impl_variant_empty!(CameraDragMove);
impl_variant_empty!(CameraDragEnd);
impl_variant_empty!(EnableSketchMode);
impl_variant_empty!(DefaultCameraLookAt);
impl_variant_empty!(DefaultCameraPerspectiveSettings);
impl_variant_empty!(DefaultCameraZoom);
impl_variant_empty!(DefaultCameraEnableSketchMode);
impl_variant_empty!(DefaultCameraDisableSketchMode);
impl_variant_empty!(DefaultCameraFocusOn);
impl_variant_output!(Export);
impl_variant_output!(EntityGetParentId);
impl_variant_output!(EntityGetNumChildren);
impl_variant_output!(EntityGetChildUuid);
impl_variant_output!(EntityGetAllChildUuids);
impl_variant_output!(EntityGetDistance);
impl_variant_output!(EntityLinearPattern);
impl_variant_output!(EntityCircularPattern);
impl_variant_empty!(EditModeEnter);
impl_variant_output!(SelectWithPoint);
impl_variant_empty!(SelectAdd);
impl_variant_empty!(SelectRemove);
impl_variant_empty!(SelectReplace);
impl_variant_output!(HighlightSetEntity);
impl_variant_empty!(HighlightSetEntities);
impl_variant_empty!(NewAnnotation);
impl_variant_empty!(UpdateAnnotation);
impl_variant_empty!(ObjectVisible);
impl_variant_empty!(ObjectBringToFront);
impl_variant_output!(GetEntityType);
impl_variant_empty!(Solid2dAddHole);
impl_variant_output!(Solid3dGetAllEdgeFaces);
impl_variant_output!(Solid3dGetAllOppositeEdges);
impl_variant_output!(Solid3dGetOppositeEdge);
impl_variant_output!(Solid3dGetNextAdjacentEdge);
impl_variant_output!(Solid3dGetPrevAdjacentEdge);
impl_variant_empty!(Solid3dFilletEdge);
impl_variant_empty!(SendObject);
impl_variant_empty!(ObjectSetMaterialParamsPbr);
impl_variant_empty!(EntitySetOpacity);
impl_variant_empty!(EntityFade);
impl_variant_empty!(MakePlane);
impl_variant_empty!(PlaneSetColor);
impl_variant_empty!(SetTool);
impl_variant_empty!(MouseMove);
impl_variant_output!(MouseClick);
impl_variant_empty!(SketchModeEnable);
impl_variant_empty!(SketchModeDisable);
impl_variant_output!(CurveGetType);
impl_variant_output!(CurveGetControlPoints);
impl_variant_output!(TakeSnapshot);
impl_variant_empty!(MakeAxesGizmo);
impl_variant_output!(PathGetInfo);
impl_variant_output!(PathGetCurveUuidsForVertices);
impl_variant_output!(PathGetVertexUuids);
impl_variant_empty!(HandleMouseDragStart);
impl_variant_empty!(HandleMouseDragMove);
impl_variant_empty!(HandleMouseDragEnd);
impl_variant_empty!(RemoveSceneObjects);
impl_variant_empty!(CurveSetConstraint);
impl_variant_empty!(ReconfigureStream);
impl_variant_output!(PlaneIntersectAndProject);
impl_variant_output!(CurveGetEndPoints);
impl_variant_output!(ImportFiles);
impl_variant_output!(Mass);
impl_variant_output!(Volume);
impl_variant_output!(Density);
impl_variant_output!(SurfaceArea);
impl_variant_output!(CenterOfMass);
impl_variant_output!(GetSketchModePlane);
impl_variant_empty!(SetSelectionFilter);
impl_variant_empty!(SetSelectionType);
impl_variant_empty!(DefaultCameraSetOrthographic);
impl_variant_empty!(DefaultCameraSetPerspective);
impl_variant_output!(Solid3dGetExtrusionFaceInfo);
impl_variant_empty!(SetSceneUnits);
