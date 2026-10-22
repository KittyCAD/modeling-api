# KittyCAD's Modeling API

This defines the API KittyCAD provides for you to model 3D objects.

# Beware

This project does not use semver. We are using 0.1.x for everything. If you use this crate, commit your Cargo.lock to avoid being broken when we publish a new version. 

Why? Because we use this primarily for KittyCAD server and clients, where we are on top of all changes.

## Drawing projection contract

`ComputeDrawingProjection` is a proposed read-only modeling command for an
orthographic projection of one BREP body. The engine computes projected curves,
silhouettes, and visibility. ZDS owns rendering, paper layout, styling, picking,
and authoring KCL references through the artifact graph and Face/Edge APIs.
Declaring the command here does not implement it in engine.

The input is the current body's `object_id`, a model-space `frame` (origin and
orthogonal unit X/Y axes), and optional hidden-line, resolution, and tolerance
overrides. The viewer looks along the negative cross product of X and Y. Other
bodies in the scene do not participate in occlusion. The frame's origin uses the
session length unit; output coordinates/radii and the tolerance use millimeters.
Defaults are both visible and hidden curves, sampling resolution 19, and
tolerance 0.00001 mm. Resolution must be at least 2 and tolerance positive and
finite. The engine must reject invalid parameters before entering the algorithm.

The response's `curves` array contains tagged line, circular-arc, or NURBS
geometry, visibility, a tagged source, and a split-segment index. All geometry is
in the projection's local XY frame, before drawing scale or paper placement.
Circular arcs use angles from local +X towards +Y, in radians; a full circle has
a 2-pi sweep. NURBS use non-homogeneous control points, optional rational weights,
order (degree + 1), and a full knot vector. These wire types have no renderer or
file-export dependency.

### Identity and the Face API

Source is an enum rather than a kind plus unrelated optional fields:

| Source | Required topology | Derived component |
| --- | --- | --- |
| `edge` | `edge_index`, optional current-execution `edge_id` | `section_index` |
| `silhouette` | `face_index`, optional current-execution `face_id` | `silhouette_index` |

Topology indices use the same BREP indexing as `EntityGetPrimitiveIndex`, on the
body named in the request. The client can map available UUIDs through its artifact
graph to tags/expressions. For untaggable topology it can author `faceId(body,
index = …)` or `edgeId(body, index = …)` and resolve them in the next execution.
The response does not duplicate that client-side work with prebuilt KCL selectors.
Topology indices are a fallback, not a promise that a selector survives arbitrary
topology-changing edits.

`section_index` distinguishes pieces of one source edge separated at view-tangent
conditions. `silhouette_index` distinguishes components generated from one face.
`segment_index` distinguishes pieces after further intersection/visibility splitting.
These values preserve the algorithm's current identification, but are local to
the projection result: no cross-version or model/view-edit stability is promised.
Use array positions for transient picking IDs. A face reference alone cannot
identify which of several silhouettes or split segments the user selected.

This first response does not promise an exhaustive set of coincident contributors
or face-derived drawing-edge groups. Clients may group returned geometry, but
cannot reconstruct provenance that the engine discarded during overlap removal.
Durable annotations targeting a particular derived silhouette need a later
resolve/disambiguation contract; that is separate from resolving its source face.

### Repository idioms and engine pairing

1. `modeling-cmds/src/def_enum.rs` declares the request inside
   `define_modeling_cmd_enum!`. Its macros generate the `ModelingCmd` union and
   command-specific plumbing. This is a message on the existing modeling
   WebSocket, not a new HTTP route.
2. `modeling-cmds/src/ok_response.rs` declares the matching result inside
   `define_ok_modeling_cmd_response_enum!`.
3. `modeling-cmds/src/shared.rs` holds the reusable payload types. Serde defines
   the JSON tags; `JsonSchema` supplies OpenAPI; optional `ts-rs` exports supply
   TypeScript. `Builder` follows the existing command-construction convention.
   `non_exhaustive` requires downstream Rust matches to allow future variants.
4. The paired engine change should extract the current `makeProjection` kernel
   from alteous's engine #4329. Map `EdgeIdentifier.edge/section` to
   `edge_index/section_index`, `SilhouetteIdentifier.face/end` to
   `face_index/silhouette_index`, and `ProjectedCurve.segment` to `segment_index`.
   Obtain available topology UUIDs from the owning BREP. Keep identification next
   to geometry through splitting and filtering. Remove debug-scene mutations and
   drawing presentation dependencies from the query path.
5. Engine #4886 is a reference for WebSocket dispatch, Rust actor, CXX bridge,
   and geometry serialization. It needs adaptation to this source enum and the
   current projection kernel, parameter checks, arc-axis conversion into the
   projection frame, and tests for units, silhouettes, hidden lines, and splitting.
   This contract's wire tests do not prove engine geometry correctness.
6. Review the two changes together. Merge and release modeling-cmds through the
   repository's existing release process, then use the released version in engine
   and regenerate/update the client bindings. A temporary dependency override is
   useful for paired development; engine CI forbids landing it.

The first ZDS consumer can call the command after KCL execution, draw the result,
and map selections back to the current body. No new KCL drawing syntax, engine-side
drawing mode, document format, or export implementation is required for that step.
