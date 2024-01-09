//! Impl Value for various container types, if the inner type implements Value.

use crate::{MemoryError, Primitive, Value};
const NONE: &str = "None";
const SOME: &str = "Some";

/// Use the standard enum convention (a string for the variant tag, then all fields of the variant)
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

/// Store the vec's length as the first primitive, then lay out all elements.
impl<T> Value for Vec<T>
where
    T: Value,
{
    fn into_parts(self) -> Vec<Primitive> {
        let mut parts: Vec<Primitive> = Vec::with_capacity(self.len() + 1);
        parts.push(self.len().into());
        parts.extend(self.into_iter().flat_map(|part| part.into_parts()));
        parts
    }

    fn from_parts<I>(values: &mut I) -> Result<Self, MemoryError>
    where
        I: Iterator<Item = Option<Primitive>>,
    {
        // Read the length of the vec -- how many elements does it have?
        let n: usize = values
            .next()
            .flatten()
            .ok_or(MemoryError::MemoryWrongSize)?
            .try_into()?;
        // Read `n` elements from the parts.
        (0..n).map(|_| T::from_parts(values)).collect()
    }
}

/// `Box<T>` is laid out identically to an unboxed `T`.
impl<T> Value for Box<T>
where
    T: Value,
{
    fn into_parts(self) -> Vec<Primitive> {
        (*self).into_parts()
    }

    fn from_parts<I>(values: &mut I) -> Result<Self, MemoryError>
    where
        I: Iterator<Item = Option<Primitive>>,
    {
        T::from_parts(values).map(Box::new)
    }
}
