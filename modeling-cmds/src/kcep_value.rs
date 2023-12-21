use kittycad_execution_plan_traits::{MemoryError, Primitive, Value};

use crate::{ok_response::OkModelingCmdResponse, output};

pub(crate) const EMPTY: &str = "EMPTY";
pub(crate) const TAKE_SNAPSHOT: &str = "TAKE_SNAPSHOT";

fn err() -> MemoryError {
    MemoryError::MemoryWrongSize
}

/// Layout:
/// - One memory address to store the variant name
/// - Following memory addresses to store the variant's single field.
impl Value for OkModelingCmdResponse {
    fn into_parts(self) -> Vec<Primitive> {
        match self {
            OkModelingCmdResponse::Empty => vec![Primitive::String(EMPTY.to_string())],
            OkModelingCmdResponse::TakeSnapshot(snap) => {
                let mut parts = vec![Primitive::String(TAKE_SNAPSHOT.to_owned())];
                parts.extend(snap.into_parts());
                parts
            }
            _ => todo!("Implement Value for more OkModelingCmdResponse variants"),
        }
    }

    fn from_parts<I>(values: &mut I) -> Result<Self, MemoryError>
    where
        I: Iterator<Item = Option<Primitive>>,
    {
        let variant_name: String = next(values)?;
        match variant_name.as_str() {
            EMPTY => Ok(OkModelingCmdResponse::Empty),
            TAKE_SNAPSHOT => {
                let contents: Vec<u8> = next(values)?;
                Ok(OkModelingCmdResponse::TakeSnapshot(output::TakeSnapshot {
                    contents: contents.into(),
                }))
            }
            _ => todo!("Implement Value for more OkModelingCmdResponse variants"),
        }
    }
}

/// Layout: A single memory address, storing the snapshot's bytes as a primitive.
impl Value for output::TakeSnapshot {
    fn into_parts(self) -> Vec<Primitive> {
        vec![Primitive::Bytes(self.contents.into())]
    }

    fn from_parts<I>(values: &mut I) -> Result<Self, MemoryError>
    where
        I: Iterator<Item = Option<Primitive>>,
    {
        let contents: Vec<u8> = next(values)?;
        Ok(Self {
            contents: contents.into(),
        })
    }
}

/// Read the next primitive.
/// If it's
fn next<I, T>(values: &mut I) -> Result<T, MemoryError>
where
    I: Iterator<Item = Option<Primitive>>,
    T: TryFrom<Primitive, Error = MemoryError>,
{
    values.next().ok_or_else(err)?.ok_or_else(err)?.try_into()
}
