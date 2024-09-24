use serde::{de::DeserializeOwned, Serialize};

use crate::ModelingCmd;

/// Some modeling command executed on the KittyCAD engine.
pub trait ModelingCmdVariant: Serialize {
    /// What the command responds with
    type Output: ModelingCmdOutput;
    /// Take this specific enum variant, and create the general enum.
    fn into_enum(self) -> ModelingCmd;
    /// The command's name.
    fn name() -> &'static str;
}

/// Anything that can be a ModelingCmd output.
pub trait ModelingCmdOutput: std::fmt::Debug + Serialize + DeserializeOwned {}

impl<CmdVariant> From<CmdVariant> for ModelingCmd
where
    CmdVariant: ModelingCmdVariant,
{
    fn from(value: CmdVariant) -> Self {
        value.into_enum()
    }
}
