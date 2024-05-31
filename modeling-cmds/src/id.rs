use schemars::JsonSchema;
use serde::{de, de::Visitor, Deserialize, Deserializer, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

/// All commands have unique IDs. These should be randomly generated.
#[derive(Debug, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq, JsonSchema, Serialize)]
#[cfg_attr(feature = "test", derive(Default))]
pub struct ModelingCmdId(pub Uuid);

// In order to force our own UUID requirements, we need to intercept /
// implement our own serde deserializer for UUID essentially. We are
// fortunate to have wrapped the UUID type already so we can do this.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modeling_cmd_id_from_bson() {
        #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
        struct Id {
            id: ModelingCmdId,
        }

        // Serializing and deserializing an ID (via BSON) should not change it.
        let id_before = Id {
            id: ModelingCmdId("f09fc20f-40d4-4a73-92fa-05d53baaabac".parse().unwrap()),
        };
        let bytes = bson::to_vec(&id_before).unwrap();
        let id_after = bson::from_reader(bytes.as_slice()).unwrap();
        assert_eq!(id_before, id_after);
    }
}

struct UuidVisitor;

impl<'de> Visitor<'de> for UuidVisitor {
    type Value = ModelingCmdId;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expected a string for uuid")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ModelingCmdId::from_str(value).map_err(|e| de::Error::custom(e.to_string()))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Uuid::from_slice(v)
            .map_err(|e| de::Error::custom(e.to_string()))
            .map(ModelingCmdId)
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Uuid::from_slice(v)
            .map_err(|e| de::Error::custom(e.to_string()))
            .map(ModelingCmdId)
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Uuid::from_slice(v.as_slice())
            .map_err(|e| de::Error::custom(e.to_string()))
            .map(ModelingCmdId)
    }
}

impl<'de> Deserialize<'de> for ModelingCmdId {
    fn deserialize<D>(deserializer: D) -> Result<ModelingCmdId, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(UuidVisitor)
    }
}

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
