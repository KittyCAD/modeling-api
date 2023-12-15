use kittycad_modeling_cmds::{
    each_cmd::{MovePathPen, StartPath},
    id::ModelingCmdId,
    ExtendPath, ModelingCmdVariant,
};
use uuid::Uuid;

use crate::{primitive::Primitive, value::Value, Address, ExecutionError, Memory, Result};

pub trait ApiEndpoint: ModelingCmdVariant + Sized {
    fn from_values<I>(values: &mut I, mem: &Memory) -> Result<Self>
    where
        I: Iterator<Item = Address>;
}

impl ApiEndpoint for StartPath {
    fn from_values<I>(_values: &mut I, _mem: &Memory) -> Result<Self>
    where
        I: Iterator<Item = Address>,
    {
        Ok(Self {})
    }
}

impl ApiEndpoint for MovePathPen {
    fn from_values<I>(values: &mut I, mem: &Memory) -> Result<Self>
    where
        I: Iterator<Item = Address>,
    {
        let path: Uuid = read::<Primitive>(values.next(), 2, mem)?.try_into()?;
        let path = ModelingCmdId::from(path);
        let to = read(values.next(), 2, mem)?;
        Ok(Self { path, to })
    }
}

impl ApiEndpoint for ExtendPath {
    fn from_values<I>(values: &mut I, mem: &Memory) -> Result<Self>
    where
        I: Iterator<Item = Address>,
    {
        let path = read::<Primitive>(values.next(), 2, mem)
            .and_then(Uuid::try_from)
            .map(ModelingCmdId::from)?;
        let segment = read(values.next(), 2, mem)?;
        Ok(Self { path, segment })
    }
}

fn read<T: Value>(start_addr: Option<Address>, expected_num: usize, mem: &Memory) -> Result<T> {
    let start_addr = start_addr.ok_or(ExecutionError::MemoryWrongSize { expected: expected_num })?;
    mem.get_composite(start_addr)
}
