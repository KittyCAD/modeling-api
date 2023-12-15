use kittycad_modeling_cmds::{
    each_cmd::{MovePathPen, StartPath},
    id::ModelingCmdId,
    ClosePath, ExtendPath, Extrude, ModelingCmdVariant, TakeSnapshot,
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
        let path: Uuid = read::<Primitive>(values.next(), mem)?.try_into()?;
        let path = ModelingCmdId::from(path);
        let to = read(values.next(), mem)?;
        Ok(Self { path, to })
    }
}

impl ApiEndpoint for ExtendPath {
    fn from_values<I>(values: &mut I, mem: &Memory) -> Result<Self>
    where
        I: Iterator<Item = Address>,
    {
        let path = read::<Primitive>(values.next(), mem)
            .and_then(Uuid::try_from)
            .map(ModelingCmdId::from)?;
        let segment = read(values.next(), mem)?;
        Ok(Self { path, segment })
    }
}

impl ApiEndpoint for Extrude {
    fn from_values<I>(values: &mut I, mem: &Memory) -> Result<Self>
    where
        I: Iterator<Item = Address>,
    {
        let target = read::<Primitive>(values.next(), mem)
            .and_then(Uuid::try_from)
            .map(ModelingCmdId)?;
        let distance = read::<Primitive>(values.next(), mem).and_then(f64::try_from)?;
        let cap = read::<Primitive>(values.next(), mem).and_then(bool::try_from)?;
        Ok(Self { target, distance, cap })
    }
}

impl ApiEndpoint for TakeSnapshot {
    fn from_values<I>(values: &mut I, mem: &Memory) -> Result<Self>
    where
        I: Iterator<Item = Address>,
    {
        let format_str = read::<Primitive>(values.next(), mem).and_then(String::try_from)?;
        let format = format_str.parse().map_err(|_| ExecutionError::InvalidEnumVariant {
            expected_type: "image format".to_owned(),
            actual: format_str,
        })?;
        Ok(Self { format })
    }
}

impl ApiEndpoint for ClosePath {
    fn from_values<I>(values: &mut I, mem: &Memory) -> Result<Self>
    where
        I: Iterator<Item = Address>,
    {
        let path_id = read::<Primitive>(values.next(), mem).and_then(Uuid::try_from)?;
        Ok(Self { path_id })
    }
}

fn read<T: Value>(start_addr: Option<Address>, mem: &Memory) -> Result<T> {
    let start_addr = start_addr.ok_or(ExecutionError::MemoryWrongSize)?;
    mem.get_composite(start_addr)
}
