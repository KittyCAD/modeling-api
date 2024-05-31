/// Import functionality.
pub mod import {
    
    use parse_display::{Display, FromStr};
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    /// Options for importing SolidWorks parts.
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        Hash,
        PartialEq,
        Serialize,
        Deserialize,
        JsonSchema,
        Display,
        FromStr,
       
    )]
    #[display("")]
    pub struct Options {}
}
