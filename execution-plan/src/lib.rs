//! A KittyCAD execution plan (KCEP) is a list of
//! - KittyCAD API requests to make
//! - Values to send in API requests
//! - Values to assign from API responses
//! - Computation to perform on values
//! You can think of it as a domain-specific language for making KittyCAD API calls and using
//! the results to make other API calls.

use events::{Event, EventWriter};
use kittycad_execution_plan_traits::{FromMemory, MemoryError, Primitive, ReadMemory};
use kittycad_modeling_cmds::{each_cmd, id::ModelingCmdId};
use kittycad_modeling_session::{RunCommandError, Session as ModelingSession};
pub use memory::{Memory, Stack, StaticMemoryInitializer};
use serde::{Deserialize, Serialize};

pub use self::address::Address;
pub use self::arithmetic::{
    operator::{BinaryOperation, Operation, UnaryOperation},
    BinaryArithmetic, UnaryArithmetic,
};
pub use self::instruction::Instruction;

mod address;
mod arithmetic;
mod events;
mod instruction;
mod memory;
#[cfg(test)]
mod tests;

/// Somewhere values can be written to.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Destination {
    /// Write to main memory at the given address.
    Address(Address),
    /// Push onto the stack.
    StackPush,
}

/// Request sent to the KittyCAD API.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
#[derive(Serialize, Deserialize, parse_display_derive::Display, Debug, PartialEq, Clone, Copy)]
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

/// Argument to an operation.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum Operand {
    /// A literal value.
    Literal(Primitive),
    /// An address which contains some literal value.
    Reference(Address),
    /// Pop the value from the top of the stack.
    StackPop,
}

impl Operand {
    /// Evaluate the operand, getting its value.
    fn eval(&self, mem: &mut Memory) -> Result<Primitive> {
        match self {
            Operand::Literal(v) => Ok(v.to_owned()),
            Operand::Reference(addr) => match mem.get(addr) {
                None => Err(ExecutionError::MemoryEmpty { addr: *addr }),
                Some(v) => Ok(v.to_owned()),
            },
            Operand::StackPop => mem.stack.pop_single(),
        }
    }
}

/// Execute the plan.
pub async fn execute(mem: &mut Memory, plan: Vec<Instruction>, mut session: Option<ModelingSession>) -> Result<()> {
    let mut events = EventWriter::default();
    for instruction in plan.into_iter() {
        instruction.execute(mem, session.as_mut(), &mut events).await?;
    }
    Ok(())
}

/// Current state of execution.
pub struct ExecutionState {
    /// State of memory after executing the instruction
    pub mem: Memory,
    /// Which instruction was executed? Index into the Vec<Instruction> for the plan.
    pub active_instruction: usize,
    /// Which events occurred during execution of this instruction?
    pub events: Vec<Event>,
}

/// Execute the plan, returning the state at every moment of execution.
pub async fn execute_time_travel(
    mem: &mut Memory,
    plan: Vec<Instruction>,
    mut session: Option<ModelingSession>,
) -> (Vec<ExecutionState>, Result<()>) {
    let mut out = Vec::new();
    let mut events = EventWriter::default();
    for (active_instruction, instruction) in plan.into_iter().enumerate() {
        let res = instruction.execute(mem, session.as_mut(), &mut events).await;
        out.push(ExecutionState {
            mem: mem.clone(),
            active_instruction,
            events: events.drain(),
        });
        if res.is_err() {
            return (out, res);
        }
    }
    (out, Ok(()))
}

type Result<T> = std::result::Result<T, ExecutionError>;

/// Errors that could occur when executing a KittyCAD execution plan.
#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    /// Stack should have contained a single primitive but it had a composite value instead.
    #[error("Expected stack to contain a single primitive, but it had a slice of length {actual_length}")]
    StackNotPrimitive {
        /// The actual size of the data that was popped off the stack
        /// Expected to be 1, but it was something else.
        actual_length: usize,
    },
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
    /// List index out of bounds.
    #[error("you tried to access element {index} in a list of size {count}")]
    ListIndexOutOfBounds {
        /// Number of elements in the list.
        count: usize,
        /// Index which user attempted to access.
        index: usize,
    },
    /// Tried to pop from empty stack.
    #[error("tried to pop from empty stack")]
    StackEmpty,
    /// Could not make API call because no KittyCAD API client was provided
    #[error("could not make API call because no KittyCAD API client was provided")]
    NoApiClient,
    /// Property not found in object.
    #[error("No property '{property}' exists in the object starting at {address}")]
    UndefinedProperty {
        /// Which property the program was trying to access.
        property: String,
        /// Starting address of the object
        address: Address,
    },
}
