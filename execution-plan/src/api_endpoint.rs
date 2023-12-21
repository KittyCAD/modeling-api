use kittycad_execution_plan_traits::{MemoryError, Value};
use kittycad_modeling_cmds::{
    each_cmd::{MovePathPen, StartPath},
    ClosePath, ExtendPath, Extrude, ModelingCmdVariant, TakeSnapshot,
};

use crate::{Address, Memory, Result};

/// All API endpoints that can be executed must implement this trait.
pub trait ApiEndpoint: ModelingCmdVariant + Sized {
    /// Read the API call and its parameters from memory.
    /// For each field in the API endpoint's body,
    /// 1. Read that field's address from the `fields` iterator.
    /// 2. Look up the value at that address
    /// Then use those fields to reconstruct the entire struct.
    fn from_memory<I>(fields: &mut I, mem: &Memory) -> Result<Self>
    where
        I: Iterator<Item = Address>;
}

impl ApiEndpoint for StartPath {
    fn from_memory<I>(_fields: &mut I, _mem: &Memory) -> Result<Self>
    where
        I: Iterator<Item = Address>,
    {
        Ok(Self {})
    }
}

impl ApiEndpoint for MovePathPen {
    fn from_memory<I>(fields: &mut I, mem: &Memory) -> Result<Self>
    where
        I: Iterator<Item = Address>,
    {
        let path = read(fields.next(), mem)?;
        let to = read(fields.next(), mem)?;
        Ok(Self { path, to })
    }
}

impl ApiEndpoint for ExtendPath {
    fn from_memory<I>(fields: &mut I, mem: &Memory) -> Result<Self>
    where
        I: Iterator<Item = Address>,
    {
        let path = read(fields.next(), mem)?;
        let segment = read(fields.next(), mem)?;
        Ok(Self { path, segment })
    }
}

impl ApiEndpoint for Extrude {
    fn from_memory<I>(fields: &mut I, mem: &Memory) -> Result<Self>
    where
        I: Iterator<Item = Address>,
    {
        let target = read(fields.next(), mem)?;
        let distance = read(fields.next(), mem)?;
        let cap = read(fields.next(), mem)?;
        Ok(Self { target, distance, cap })
    }
}

impl ApiEndpoint for TakeSnapshot {
    fn from_memory<I>(fields: &mut I, mem: &Memory) -> Result<Self>
    where
        I: Iterator<Item = Address>,
    {
        let format = read(fields.next(), mem)?;
        Ok(Self { format })
    }
}

impl ApiEndpoint for ClosePath {
    fn from_memory<I>(fields: &mut I, mem: &Memory) -> Result<Self>
    where
        I: Iterator<Item = Address>,
    {
        let path_id = read(fields.next(), mem)?;
        Ok(Self { path_id })
    }
}

fn read<T: Value>(start_addr: Option<Address>, mem: &Memory) -> std::result::Result<T, MemoryError> {
    start_addr
        .ok_or(MemoryError::MemoryWrongSize)
        .and_then(|a| mem.get_composite(a))
}
