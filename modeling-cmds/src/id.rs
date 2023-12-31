use kittycad_execution_plan_traits::{MemoryError, Primitive};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// All commands have unique IDs. These should be randomly generated.
#[derive(Debug, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq, JsonSchema, Deserialize, Serialize)]
#[cfg_attr(feature = "test", derive(Default))]
pub struct ModelingCmdId(pub Uuid);

impl std::str::FromStr for ModelingCmdId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s).map(Self)
    }
}

impl From<Uuid> for ModelingCmdId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<ModelingCmdId> for Uuid {
    fn from(id: ModelingCmdId) -> Self {
        id.0
    }
}

impl std::fmt::Display for ModelingCmdId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<ModelingCmdId> for Primitive {
    fn from(id: ModelingCmdId) -> Self {
        Self::Uuid(id.into())
    }
}

impl TryFrom<Primitive> for ModelingCmdId {
    type Error = MemoryError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        if let Primitive::Uuid(u) = value {
            Ok(u.into())
        } else {
            Err(MemoryError::MemoryWrongType {
                expected: "uuid",
                actual: format!("{value:?}"),
            })
        }
    }
}
