//! KittyCAD's Modeling API lets you design 3D models.
//! # Beware
//! This project does not use semver. We are using 0.1.x for everything. If you use this crate, commit your Cargo.lock to avoid being broken when we publish a new version.
//! Why? Because we use this primarily for KittyCAD server and clients, where we are on top of all changes.

pub mod base64;

#[cfg(feature = "convert_client_crate")]
mod convert_client_crate;

/// Various coordinate systems.
pub mod coord;

/// chrono wrapper for datetimes.
pub mod datetime;

/// The modeling command enum with each specific modeling command.
mod def_enum;

/// Import and export types.
pub mod format;

/// Modeling command IDs, used to associated requests and responses.
/// Also used to construct commands which refer to previous commands.
pub mod id;

#[cfg(feature = "cxx")]
pub mod impl_extern_type;

pub mod length_unit;

/// When a modeling command is successful, these responses could be returned.
pub mod ok_response;

/// Controlling the rendering session.
pub mod session;

/// Types that are shared between various modeling commands, like Point3d.
pub mod shared;

#[cfg(all(test, feature = "derive-jsonschema-on-enums"))]
mod tests;

/// The modeling command trait that each modeling command implements.
mod traits;

/// Units of measurement.
pub mod units;

/// Types for the WebSocket API.
#[cfg(feature = "websocket")]
pub mod websocket;

// Export some key items for anyone consuming the library.
pub use def_enum::*;
pub use ok_response::output;
pub use traits::*;
