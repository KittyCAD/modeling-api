//! KittyCAD's Modeling API lets you design 3D models.

pub mod base64;
/// Various coordinate systems.
pub mod coord;
/// The modeling command enum with each specific modeling command.
mod def_enum;
/// Definition of each modeling command.
pub mod each_cmd;
/// Import and export types.
pub mod format;
/// Modeling command IDs, used to associated requests and responses.
/// Also used to construct commands which refer to previous commands.
pub mod id;
#[cfg(feature = "cxx")]
pub mod impl_extern_type;
mod impl_traits;
mod kcep_primitive;
pub mod length_unit;
/// When a modeling command is successful, these responses could be returned.
pub mod ok_response;
/// Output of each modeling command.
pub mod output;
/// Types that are shared between various modeling commands, like Point3d.
pub mod shared;
/// The modeling command trait that each modeling command implements.
mod traits;
/// Units of measurement.
pub mod units;
/// Types for the WebSocket API.
#[cfg(feature = "websocket")]
pub mod websocket;

pub use def_enum::*;
pub use traits::*;
