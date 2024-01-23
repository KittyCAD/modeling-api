//! A KittyCAD execution plan (KCEP) is a list of
//! - KittyCAD API requests to make
//! - Values to send in API requests
//! - Values to assign from API responses
//! - Computation to perform on values
//! You can think of it as a domain-specific language for making KittyCAD API calls and using
//! the results to make other API calls.

use std::fmt;

use kittycad_execution_plan_traits::{FromMemory, MemoryError, Primitive, ReadMemory};
use kittycad_modeling_cmds::{each_cmd, id::ModelingCmdId};
use kittycad_modeling_session::{RunCommandError, Session as ModelingSession};
pub use memory::{Memory, StaticMemoryInitializer};
use serde::{Deserialize, Serialize};

pub use self::arithmetic::BinaryArithmetic;

mod arithmetic;
mod memory;
#[cfg(test)]
mod tests;

/// An address in KCEP's program memory.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Address(usize);

impl Address {
    /// First memory address available.
    pub const ZERO: Self = Self(0);

    /// Offset the memory by `size` addresses.
    pub fn offset(self, size: usize) -> Self {
        let curr = self.0;
        Self(curr + size)
    }

    /// Returns self, then offsets self by `size` addresses.
    pub fn offset_by(&mut self, size: usize) -> Self {
        let old = *self;
        self.0 += size;
        old
    }
}

/// Offset the address.
impl std::ops::Add<usize> for Address {
    type Output = Self;

    /// Offset the address.
    fn add(self, rhs: usize) -> Self::Output {
        self.offset(rhs)
    }
}

/// Offset the address.
impl std::ops::Add for Address {
    type Output = Self;

    /// Offset the address.
    fn add(self, rhs: Self) -> Self::Output {
        self.offset(rhs.0)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<usize> for Address {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

/// One step of the execution plan.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Instruction {
    /// Call the KittyCAD API.
    ApiRequest(ApiRequest),
    /// Set a primitive to a memory address.
    SetPrimitive {
        /// Which memory address to set.
        address: Address,
        /// What value to set the memory address to.
        value: Primitive,
    },
    /// Lay out a multi-address value in memory.
    SetValue {
        /// Which memory address to set.
        address: Address,
        /// What values to put into memory.
        value_parts: Vec<Primitive>,
    },
    /// Perform arithmetic on values in memory.
    Arithmetic {
        /// What to do.
        arithmetic: BinaryArithmetic,
        /// Write the output to this memory address.
        destination: Address,
    },
}

/// Request sent to the KittyCAD API.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ApiRequest {
    /// Which ModelingCmd to call.
    pub endpoint: Endpoint,
    /// Which address should the response be stored in?
    /// If none, the response will be ignored.
    pub store_response: Option<Address>,
    /// Look up each parameter at this address.
    pub arguments: Vec<Address>,
    /// The ID of this command.
    pub cmd_id: ModelingCmdId,
}

/// A KittyCAD modeling command.
#[derive(Serialize, Deserialize, parse_display_derive::Display, Debug, PartialEq)]
pub enum Endpoint {
    #[allow(missing_docs)]
    StartPath,
    #[allow(missing_docs)]
    MovePathPen,
    #[allow(missing_docs)]
    ExtendPath,
    #[allow(missing_docs)]
    ClosePath,
    #[allow(missing_docs)]
    Extrude,
    #[allow(missing_docs)]
    TakeSnapshot,
}

impl ApiRequest {
    async fn execute(self, session: &mut ModelingSession, mem: &mut Memory) -> Result<()> {
        let Self {
            endpoint,
            store_response,
            arguments,
            cmd_id,
        } = self;
        let mut arguments = arguments.into_iter();
        let output = match endpoint {
            Endpoint::StartPath => {
                let cmd = each_cmd::StartPath::from_memory(&mut arguments, mem)?;
                session.run_command(cmd_id, cmd).await?
            }
            Endpoint::MovePathPen => {
                let cmd = each_cmd::MovePathPen::from_memory(&mut arguments, mem)?;
                session.run_command(cmd_id, cmd).await?
            }
            Endpoint::ExtendPath => {
                let cmd = each_cmd::ExtendPath::from_memory(&mut arguments, mem)?;
                session.run_command(cmd_id, cmd).await?
            }
            Endpoint::ClosePath => {
                let cmd = each_cmd::ClosePath::from_memory(&mut arguments, mem)?;
                session.run_command(cmd_id, cmd).await?
            }
            Endpoint::Extrude => {
                let cmd = each_cmd::Extrude::from_memory(&mut arguments, mem)?;
                session.run_command(cmd_id, cmd).await?
            }
            Endpoint::TakeSnapshot => {
                let cmd = each_cmd::TakeSnapshot::from_memory(&mut arguments, mem)?;
                session.run_command(cmd_id, cmd).await?
            }
        };
        // Write out to memory.
        if let Some(output_address) = store_response {
            mem.set_composite(output_address, output);
        }
        Ok(())
    }
}

/// Operations that can be applied to values in memory.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    /// Addition
    Add,
    /// Multiplication
    Mul,
    /// Subtraction
    Sub,
    /// Division
    Div,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Add => "+",
            Operation::Mul => "*",
            Operation::Sub => "-",
            Operation::Div => "/",
        }
        .fmt(f)
    }
}

/// Argument to an operation.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum Operand {
    /// A literal value.
    Literal(Primitive),
    /// An address which contains some literal value.
    Reference(Address),
}

impl Operand {
    /// Evaluate the operand, getting its value.
    fn eval(&self, mem: &Memory) -> Result<Primitive> {
        match self {
            Operand::Literal(v) => Ok(v.to_owned()),
            Operand::Reference(addr) => match mem.get(addr) {
                None => Err(ExecutionError::MemoryEmpty { addr: *addr }),
                Some(v) => Ok(v.to_owned()),
            },
        }
    }
}

/// Execute the plan.
pub async fn execute(mem: &mut Memory, plan: Vec<Instruction>, mut session: ModelingSession) -> Result<()> {
    for step in plan.into_iter() {
        match step {
            Instruction::ApiRequest(req) => {
                req.execute(&mut session, mem).await?;
            }
            Instruction::SetPrimitive { address, value } => {
                mem.set(address, value);
            }
            Instruction::SetValue { address, value_parts } => {
                value_parts.into_iter().enumerate().for_each(|(i, part)| {
                    mem.set(address.offset(i), part);
                });
            }
            Instruction::Arithmetic {
                arithmetic,
                destination,
            } => {
                let out = arithmetic.calculate(mem)?;
                mem.set(destination, out);
            }
        }
    }
    Ok(())
}

type Result<T> = std::result::Result<T, ExecutionError>;

/// Errors that could occur when executing a KittyCAD execution plan.
#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    /// Memory address was not set.
    #[error("Memory address {addr} was not set")]
    MemoryEmpty {
        /// Which address was missing
        addr: Address,
    },
    /// Type error, cannot apply the operation to the given operands.
    #[error("Cannot apply operation {op} to operands {operands:?}")]
    CannotApplyOperation {
        /// Operation being attempted
        op: Operation,
        /// Operands being attempted
        operands: Vec<Primitive>,
    },
    /// You tried to call a KittyCAD endpoint that doesn't exist or isn't implemented.
    #[error("No endpoint {name} recognized")]
    UnrecognizedEndpoint {
        /// Endpoint name being attempted.
        name: String,
    },
    /// Error running a modeling command.
    #[error("Error sending command to API: {0}")]
    ModelingApiError(#[from] RunCommandError),
    /// Error reading value from memory.
    #[error("{0}")]
    MemoryError(#[from] MemoryError),
}
