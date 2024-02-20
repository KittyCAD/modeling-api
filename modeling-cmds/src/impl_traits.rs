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

impl_variant_output!(CenterOfMass);
impl_variant_output!(CurveGetControlPoints);
impl_variant_output!(CurveGetEndPoints);
impl_variant_output!(CurveGetType);
impl_variant_output!(Density);
impl_variant_output!(EntityCircularPattern);
impl_variant_output!(EntityGetAllChildUuids);
impl_variant_output!(EntityGetChildUuid);
impl_variant_output!(EntityGetDistance);
impl_variant_output!(EntityGetNumChildren);
impl_variant_output!(EntityGetParentId);
impl_variant_output!(EntityLinearPattern);
impl_variant_output!(Export);
impl_variant_output!(GetEntityType);
impl_variant_output!(GetSketchModePlane);
impl_variant_output!(HighlightSetEntity);
impl_variant_output!(ImportFiles);
impl_variant_output!(Mass);
impl_variant_output!(MouseClick);
impl_variant_output!(PathGetCurveUuidsForVertices);
impl_variant_output!(PathGetInfo);
impl_variant_output!(PathGetVertexUuids);
impl_variant_output!(PlaneIntersectAndProject);
impl_variant_output!(SelectWithPoint);
impl_variant_output!(Solid3dGetAllEdgeFaces);
impl_variant_output!(Solid3dGetAllOppositeEdges);
impl_variant_output!(Solid3dGetExtrusionFaceInfo);
impl_variant_output!(Solid3dGetNextAdjacentEdge);
impl_variant_output!(Solid3dGetOppositeEdge);
impl_variant_output!(Solid3dGetPrevAdjacentEdge);
impl_variant_output!(SurfaceArea);
impl_variant_output!(TakeSnapshot);
impl_variant_output!(Volume);
