use serde::Serialize;

use crate::ModelingCmd;

/// Some modeling command executed on the KittyCAD engine.
pub trait ModelingCmdVariant<'de> {
    /// What the command responds with
    type Output: ModelingCmdOutput<'de>;
    /// Take this specific enum variant, and create the general enum.
    fn into_enum(self) -> ModelingCmd;
}

/// Anything that can be a ModelingCmd output.
pub trait ModelingCmdOutput<'de>: std::fmt::Debug + Serialize + serde::Deserialize<'de> + schemars::JsonSchema {}

impl<'de, CmdVariant> From<CmdVariant> for ModelingCmd
where
    CmdVariant: ModelingCmdVariant<'de>,
{
    fn from(value: CmdVariant) -> Self {
        value.into_enum()
    }
}
