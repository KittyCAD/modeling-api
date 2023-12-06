use parse_display::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod import {
    use super::*;

    /// Options for importing glTF 2.0.
    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr)]
    #[display("")]
    #[serde(rename = "GltfImportOptions")]
    pub struct Options {}
}

pub mod export {
    use super::*;
    /// Options for exporting glTF 2.0.
    #[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr)]
    #[display("storage: {storage}, presentation: {presentation}")]
    #[serde(rename = "GltfExportOptions")]
    pub struct Options {
        /// Specifies which kind of glTF 2.0 will be exported.
        pub storage: Storage,
        /// Specifies how the JSON will be presented.
        pub presentation: Presentation,
    }

    /// Describes the storage format of a glTF 2.0 scene.
    #[derive(
        Default, Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr,
    )]
    #[display(style = "snake_case")]
    #[serde(rename = "GltfStorage", rename_all = "snake_case")]
    pub enum Storage {
        /// Binary glTF 2.0.
        ///
        /// This is a single binary with .glb extension.
        Binary,

        /// Standard glTF 2.0.
        ///
        /// This is a JSON file with .gltf extension paired with a separate
        /// binary blob file with .bin extension.
        Standard,

        /// Embedded glTF 2.0.
        ///
        /// Single JSON file with .gltf extension binary data encoded as
        /// base64 data URIs.
        ///
        /// This is the default setting.
        #[default]
        Embedded,
    }

    /// Describes the presentation style of the glTF JSON.
    #[derive(
        Default, Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr,
    )]
    #[display(style = "snake_case")]
    #[serde(rename = "GltfPresentation", rename_all = "snake_case")]
    pub enum Presentation {
        /// Condense the JSON into the smallest possible size.
        Compact,

        /// Expand the JSON into a more human readable format.
        ///
        /// This is the default setting.
        #[default]
        Pretty,
    }
}
