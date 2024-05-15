use kittycad_execution_plan_traits::{MemoryError, Primitive};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// All commands have unique IDs. These should be randomly generated.
#[derive(Debug, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq, JsonSchema, Deserialize, Serialize)]
#[cfg_attr(feature = "test", derive(Default))]
pub struct ModelingCmdId(pub Uuid);

// causes uuid::Error(uuid::error::ErrorKind::GroupLength { group: 0, len: 0, index: 1 })
const ERR_GROUP_LENGTH: &str = "----";

impl std::str::FromStr for ModelingCmdId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // The following would be unnecessary if uuid::parser was public.
        // The uuid crate does not require hyphens. Since we return the same
        // UUID in various places, this leads to a different representation.
        // For example, 01234567890123456789012345678901 is returned as
        // 01234567-8901-2345-6789-012345678901. This is not great when
        // developers expect their UUIDs to not change (even in representation).
        // Forcing them to use hyphenated UUIDs resolves the issue.
        // 8-4-4-4-12 is the grouping.
        // uuid::error is a private module, so we have no access to ErrorKind.
        // We must use another way to invoke a uuid::Error.
        //
        let s2 = if s.len() == 32 { ERR_GROUP_LENGTH } else { s };
        Uuid::from_str(s2).map(Self)
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

#[test]
fn smoke_test() {
    use std::str::FromStr;
    assert_eq!(
        ModelingCmdId::from_str("00000000-0000-0000-0000-000000000000"),
        Ok(ModelingCmdId(
            Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap()
        ))
    );
}

#[test]
fn requires_hyphens() {
    use std::str::FromStr;
    assert_ne!(
        ModelingCmdId::from_str("00000000000000000000000000000000"),
        Ok(ModelingCmdId(
            Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap()
        ))
    );
}
