/// Some modeling command executed on the KittyCAD engine.
pub trait ModelingCmdVariant<'de> {
    /// What the command responds with
    type Output: ModelingCmdOutput<'de>;
}

/// Anything that can be a ModelingCmd output.
pub trait ModelingCmdOutput<'de>:
    std::fmt::Debug + serde::Serialize + serde::Deserialize<'de> + schemars::JsonSchema
{
}
