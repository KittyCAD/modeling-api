/// Import functionality.
pub mod import {
    use kittycad_execution_plan_macros::ExecutionPlanValue;
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
        ExecutionPlanValue,
    )]
    #[display("")]
    pub struct Options {}
}
