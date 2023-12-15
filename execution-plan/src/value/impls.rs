use kittycad_modeling_cmds::{id::ModelingCmdId, shared::Point3d, MovePathPen};
use uuid::Uuid;

use super::Value;
use crate::{Address, ExecutionError, Primitive};

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

const START_PATH: &str = "StartPath";
const MOVE_PATH_PEN: &str = "MovePathPen";

impl Value for MovePathPen {
    fn into_parts(self) -> Vec<Primitive> {
        let MovePathPen { path, to } = self;
        let to = to.into_parts();
        let mut vals = Vec::with_capacity(1 + to.len());
        vals.push(Primitive::Uuid(path.into()));
        vals.extend(to);
        vals
    }

    fn from_parts(values: &[Option<Primitive>]) -> Result<Self, ExecutionError> {
        let path = get_some(values, 0)?;
        let path = Uuid::try_from(path)?;
        let path = ModelingCmdId::from(path);
        let to = Point3d::from_parts(&values[1..])?;
        let params = MovePathPen { path, to };
        Ok(params)
    }
}

impl Value for kittycad_modeling_cmds::ok_response::OkModelingCmdResponse {
    fn into_parts(self) -> Vec<Primitive> {
        todo!()
    }

    fn from_parts(values: &[Option<Primitive>]) -> Result<Self, ExecutionError> {
        todo!()
    }
}

fn get_some(values: &[Option<Primitive>], i: usize) -> Result<Primitive, ExecutionError> {
    let addr = Address(0); // TODO: pass the `start` addr in
    let v = values.get(i).ok_or(ExecutionError::MemoryEmpty { addr })?.to_owned();
    let v = v.ok_or(ExecutionError::MemoryEmpty { addr })?.to_owned();
    Ok(v)
}
