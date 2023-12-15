use kittycad_modeling_cmds::{ok_response::OkModelingCmdResponse, output};

use super::Value;
use crate::{Address, ExecutionError, Primitive};

const EMPTY: &str = "EMPTY";
const TAKE_SNAPSHOT: &str = "TAKE_SNAPSHOT";

impl<T> Value for kittycad_modeling_cmds::shared::Point3d<T>
where
    Primitive: From<T>,
    T: TryFrom<Primitive, Error = ExecutionError>,
{
    fn into_parts(self) -> Vec<Primitive> {
        let points = [self.x, self.y, self.z];
        points.into_iter().map(|component| component.into()).collect()
    }

    fn from_parts(values: &[Option<Primitive>]) -> Result<Self, ExecutionError> {
        let err = || ExecutionError::MemoryWrongSize { expected: 3 };
        let [x, y, z] = [0, 1, 2].map(|n| values.get(n).ok_or(err()));
        let x = x?
            .to_owned()
            .ok_or(ExecutionError::MemoryWrongSize { expected: 3 })?
            .try_into()?;
        let y = y?
            .to_owned()
            .ok_or(ExecutionError::MemoryWrongSize { expected: 3 })?
            .try_into()?;
        let z = z?
            .to_owned()
            .ok_or(ExecutionError::MemoryWrongSize { expected: 3 })?
            .try_into()?;
        Ok(Self { x, y, z })
    }
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
            _ => todo!(),
        }
    }

    fn from_parts(values: &[Option<Primitive>]) -> Result<Self, ExecutionError> {
        let variant_name: String = get_some(values, 0)?.try_into()?;
        match variant_name.as_str() {
            EMPTY => Ok(OkModelingCmdResponse::Empty),
            TAKE_SNAPSHOT => {
                let contents: Vec<u8> = get_some(values, 1)?.try_into()?;
                Ok(OkModelingCmdResponse::TakeSnapshot(output::TakeSnapshot {
                    contents: contents.into(),
                }))
            }
            _ => todo!(),
        }
    }
}

/// Layout: A single memory address, storing the snapshot's bytes as a primitive.
impl Value for output::TakeSnapshot {
    fn into_parts(self) -> Vec<Primitive> {
        vec![Primitive::Bytes(self.contents.into())]
    }

    fn from_parts(values: &[Option<Primitive>]) -> Result<Self, ExecutionError> {
        let contents: Vec<u8> = get_some(values, 0)?.try_into()?;
        Ok(Self {
            contents: contents.into(),
        })
    }
}

fn get_some(values: &[Option<Primitive>], i: usize) -> Result<Primitive, ExecutionError> {
    let addr = Address(0); // TODO: pass the `start` addr in
    let v = values.get(i).ok_or(ExecutionError::MemoryEmpty { addr })?.to_owned();
    let v = v.ok_or(ExecutionError::MemoryEmpty { addr })?.to_owned();
    Ok(v)
}
