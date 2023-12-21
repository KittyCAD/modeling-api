//! Impl Value for various container types, if the inner type implements Value.

use crate::{MemoryError, Primitive, Value};
const NONE: &str = "None";
const SOME: &str = "Some";

impl<T> Value for Option<T>
where
    T: Value,
{
    fn into_parts(self) -> Vec<Primitive> {
        match self {
            Some(v) => {
                let mut parts = Vec::new();
                parts.push(SOME.to_owned().into());
                parts.extend(v.into_parts());
                parts
            }
            None => vec![NONE.to_owned().into()],
        }
    }

    fn from_parts<I>(values: &mut I) -> Result<Self, MemoryError>
    where
        I: Iterator<Item = Option<Primitive>>,
    {
        let variant: String = values
            .next()
            .flatten()
            .ok_or(MemoryError::MemoryWrongSize)?
            .try_into()?;
        match variant.as_str() {
            NONE => Ok(None),
            SOME => {
                let val = T::from_parts(values)?;
                Ok(Some(val))
            }
            other => Err(MemoryError::InvalidEnumVariant {
                expected_type: "option".to_owned(),
                actual: other.to_owned(),
            }),
        }
    }
}
