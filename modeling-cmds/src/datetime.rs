use kittycad_execution_plan_traits::{MemoryError, NumericPrimitive, Primitive, Value};

/// A wrapper for chrono types, since we need to impl Value for them.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct DateTimeLocal {
    value: chrono::DateTime<chrono::Local>,
}

impl Value for DateTimeLocal {
    fn into_parts(self) -> Vec<Primitive> {
        vec![Primitive::NumericValue(NumericPrimitive::Integer(
            self.value.timestamp(),
        ))]
    }

    /// Read the value from memory.
    fn from_parts<I>(values: &mut I) -> Result<(Self, usize), MemoryError>
    where
        I: Iterator<Item = Option<Primitive>>,
    {
        let maybe_datetime = values.next().unwrap();

        match maybe_datetime {
            None => Ok((
                DateTimeLocal {
                    value: chrono::DateTime::from_timestamp(0, 0).unwrap().into(),
                },
                1,
            )),
            Some(Primitive::NumericValue(NumericPrimitive::Integer(timestamp_sec))) => Ok((
                DateTimeLocal {
                    value: chrono::DateTime::from_timestamp(timestamp_sec, 0).unwrap().into(),
                },
                1,
            )),
            Some(o) => Err(MemoryError::MemoryWrongType {
                expected: "i64 numeric timestamp expected",
                actual: format!("{:?}", o),
            }),
        }
    }
}
