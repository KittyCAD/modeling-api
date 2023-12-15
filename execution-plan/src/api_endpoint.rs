use kittycad_modeling_cmds::{
    each_cmd::{MovePathPen, StartPath},
    id::ModelingCmdId,
    ModelingCmdVariant,
};
use uuid::Uuid;

use crate::{primitive::Primitive, value::Value, Address, ExecutionError, Memory, Result};

pub trait ApiEndpoint: ModelingCmdVariant + Sized {
    fn from_values(values: Vec<Address>, mem: &Memory) -> Result<Self>;
}

impl ApiEndpoint for StartPath {
    fn from_values(_: Vec<Address>, _: &Memory) -> Result<Self> {
        Ok(Self {})
    }
}

impl ApiEndpoint for MovePathPen {
    fn from_values(values: Vec<Address>, mem: &Memory) -> Result<Self> {
        let mut values = values.into_iter();
        let path: Uuid = read::<Primitive>(values.next(), 2, mem)?.try_into()?;
        let path = ModelingCmdId::from(path);
        let to = read(values.next(), 2, mem)?;
        Ok(Self { path, to })
    }
}

fn read<T: Value>(start_addr: Option<Address>, expected_num: usize, mem: &Memory) -> Result<T> {
    let start_addr = start_addr.ok_or(ExecutionError::MemoryWrongSize { expected: expected_num })?;
    mem.get_composite(start_addr)
}
