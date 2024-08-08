/// Import functionality.
pub mod import {

    use parse_display::{Display, FromStr};
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    /// Options for importing SolidWorks parts.
    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr)]
    #[display("split_closed_faces: {split_closed_faces}")]
    #[serde(default, rename = "SldprtImportOptions")]
    pub struct Options {
        /// Splits all closed faces into two open faces.
        ///
        /// Defaults to `false` but is implicitly `true` when importing into the engine.
        pub split_closed_faces: bool,
    }
}
