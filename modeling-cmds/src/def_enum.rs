use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use crate::each_cmd::*;

/// Commands that the KittyCAD engine can execute.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ModelingCmd {
    /// Start a path.
    StartPath(StartPath),
    /// Move the path's "pen".
    MovePathPen(MovePathPen),
    /// Extend a path by adding a new segment which starts at the path's "pen".
    /// If no "pen" location has been set before (via `MovePen`), then the pen is at the origin.
    ExtendPath(ExtendPath),
    /// Extrude a 2D solid.
    Extrude(Extrude),
    /// Closes a path, converting it to a 2D solid.
    ClosePath(ClosePath),
    /// Camera drag started.
    CameraDragStart(CameraDragStart),
    /// Camera drag continued.
    CameraDragMove(CameraDragMove),
    /// Camera drag ended.
    CameraDragEnd(CameraDragEnd),
    /// Change what the default camera is looking at.
    DefaultCameraLookAt(DefaultCameraLookAt),
    /// Adjust zoom of the default camera.
    DefaultCameraZoom(DefaultCameraZoom),
    /// Enable sketch mode, where users can sketch 2D geometry.
    /// Users choose a plane to sketch on.
    DefaultCameraEnableSketchMode(DefaultCameraEnableSketchMode),
    /// Disable sketch mode, from the default camera.
    DefaultCameraDisableSketchMode(DefaultCameraDisableSketchMode),
    /// Focus default camera on object.
    DefaultCameraFocusOn(DefaultCameraFocusOn),
    /// Export the scene to a file.
    Export(Export),
    /// What is this entity's parent?
    EntityGetParentId(EntityGetParentId),
    /// How many children does the entity have?
    EntityGetNumChildren(EntityGetNumChildren),
    /// What is the UUID of this entity's n-th child?
    EntityGetChildUuid(EntityGetChildUuid),
    /// What are all UUIDs of this entity's children?
    EntityGetAllChildUuids(EntityGetAllChildUuids),
    /// Enter edit mode
    EditModeEnter(EditModeEnter),
    /// Exit edit mode
    EditModeExit,
    /// Modifies the selection by simulating a "mouse click" at the given x,y window coordinate
    /// Returns ID of whatever was selected.
    SelectWithPoint(SelectWithPoint),
    /// Clear the selection
    SelectClear,
    /// Adds one or more entities (by UUID) to the selection.
    SelectAdd(SelectAdd),
    /// Removes one or more entities (by UUID) from the selection.
    SelectRemove(SelectRemove),
    /// Replaces the current selection with these new entities (by UUID).
    /// Equivalent to doing SelectClear then SelectAdd.
    SelectReplace(SelectReplace),
    /// Find all IDs of selected entities
    SelectGet,
    /// Changes the current highlighted entity to whichever one is at the given window coordinate.
    /// If there's no entity at this location, clears the highlight.
    HighlightSetEntity(HighlightSetEntity),
    /// Changes the current highlighted entity to these entities.
    HighlightSetEntities(HighlightSetEntities),
    /// Create a new annotation
    NewAnnotation(NewAnnotation),
    /// Update an annotation
    UpdateAnnotation(UpdateAnnotation),
    /// Hide or show an object
    ObjectVisible(ObjectVisible),
    /// Bring an object to the front of the scene
    ObjectBringToFront(ObjectBringToFront),
    /// What type of entity is this?
    GetEntityType(GetEntityType),
    /// Add a hole to a Solid2d object before extruding it.
    Solid2dAddHole(Solid2dAddHole),
    /// Gets all faces which use the given edge.
    Solid3dGetAllEdgeFaces(Solid3dGetAllEdgeFaces),
    /// Gets all edges which are opposite the given edge, across all possible faces.
    Solid3dGetAllOppositeEdges(Solid3dGetAllOppositeEdges),
    /// Gets the edge opposite the given edge, along the given face.
    Solid3dGetOppositeEdge(Solid3dGetOppositeEdge),
    /// Gets the next adjacent edge for the given edge, along the given face.
    Solid3dGetNextAdjacentEdge(Solid3dGetNextAdjacentEdge),
    /// Gets the previous adjacent edge for the given edge, along the given face.
    Solid3dGetPrevAdjacentEdge(Solid3dGetPrevAdjacentEdge),
    /// Sends object to front or back.
    SendObject(SendObject),
    /// Set opacity of the entity.
    EntitySetOpacity(EntitySetOpacity),
    /// Fade the entity in or out.
    EntityFade(EntityFade),
    /// Make a plane.
    MakePlane(MakePlane),
    /// Set the plane's color.
    PlaneSetColor(PlaneSetColor),
    /// Set the active tool.
    SetTool(SetTool),
    /// Send a mouse move event.
    MouseMove(MouseMove),
    /// Send a mouse click event.
    /// Updates modified/selected entities.
    MouseClick(MouseClick),
    /// Enable sketch mode on the given plane.
    SketchModeEnable(SketchModeEnable),
    /// Disable sketch mode.
    SketchModeDisable(SketchModeDisable),
    /// Get type of a given curve.
    CurveGetType(CurveGetType),
    /// Get control points of a given curve.
    CurveGetControlPoints(CurveGetControlPoints),
    /// Take a snapshot.
    TakeSnapshot(TakeSnapshot),
    /// Add a gizmo showing the axes.
    MakeAxesGizmo(MakeAxesGizmo),
    /// Query the given path
    PathGetInfo(PathGetInfo),
    /// Get curves for vertices within a path
    PathGetCurveUuidsForVertices(PathGetCurveUuidsForVertices),
    /// Get vertices within a path
    PathGetVertexUuids(PathGetVertexUuids),
    /// Start dragging mouse.
    HandleMouseDragStart(HandleMouseDragStart),
    /// Continue dragging mouse.
    HandleMouseDragMove(HandleMouseDragMove),
    /// Stop dragging mouse.
    HandleMouseDragEnd(HandleMouseDragEnd),
    /// Remove scene objects.
    RemoveSceneObjects(RemoveSceneObjects),
    /// Utility method. Performs both a ray cast and projection to plane-local coordinates.
    /// Returns the plane coordinates for the given window coordinates.
    PlaneIntersectAndProject(PlaneIntersectAndProject),
    /// Find the start and end of a curve.
    CurveGetEndPoints(CurveGetEndPoints),
    /// Reconfigure the stream.
    ReconfigureStream(ReconfigureStream),
    /// Import files to the current model.
    ImportFiles(ImportFiles),
    /// Get the mass of entities in the scene or the default scene.
    Mass(Mass),
    /// Get the density of entities in the scene or the default scene.
    Density(Density),
    /// Get the volume of entities in the scene or the default scene.
    Volume(Volume),
    /// Get the center of mass of entities in the scene or the default scene.
    CenterOfMass(CenterOfMass),
    /// Get the surface area of entities in the scene or the default scene.
    SurfaceArea(SurfaceArea),
    /// Get the plane of the sketch mode. This is useful for getting the normal of the plane after
    /// a user selects a plane.
    GetSketchModePlane(GetSketchModePlane),
    /// Constrain a curve.
    CurveSetConstraint(CurveSetConstraint),
    /// Sketch on some entity (e.g. a plane, a face)
    EnableSketchMode(EnableSketchMode),
    /// Set the material properties of an object
    ObjectSetMaterialParamsPbr(ObjectSetMaterialParamsPbr),
    /// What is the distance between these two entities?
    EntityGetDistance(EntityGetDistance),
    /// Duplicate the given entity, evenly spaced along the chosen axis.
    EntityLinearPattern(EntityLinearPattern),
}

impl ModelingCmd {
    /// Is this command safe to run in an engine batch?
    pub fn is_safe_to_batch(&self) -> bool {
        use ModelingCmd::*;
        matches!(
            self,
            MovePathPen(_)
                | ExtendPath(_)
                | Extrude(_)
                | ClosePath(_)
                | UpdateAnnotation(_)
                | ObjectVisible(_)
                | ObjectBringToFront(_)
                | Solid2dAddHole(_)
                | SendObject(_)
                | EntitySetOpacity(_)
                | PlaneSetColor(_)
                | SetTool(_)
        )
    }
}
