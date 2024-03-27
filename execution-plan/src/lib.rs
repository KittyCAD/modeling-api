//! A KittyCAD execution plan (KCEP) is a list of
//! - KittyCAD API requests to make
//! - Values to send in API requests
//! - Values to assign from API responses
//! - Computation to perform on values
//! You can think of it as a domain-specific language for making KittyCAD API calls and using
//! the results to make other API calls.

use events::{Event, EventWriter};
use kittycad_execution_plan_traits::events;
use kittycad_execution_plan_traits::Address;
use kittycad_execution_plan_traits::{MemoryError, Primitive, ReadMemory};
use kittycad_modeling_cmds::websocket::ModelingBatch;
use kittycad_modeling_session::{RunCommandError, Session as ModelingSession};
pub use memory::{Memory, Stack, StaticMemoryInitializer};
use serde::{Deserialize, Serialize};

use self::api_request::ApiRequest;
pub use self::arithmetic::{
    operator::{BinaryOperation, Operation, UnaryOperation},
    BinaryArithmetic, UnaryArithmetic,
};
use self::import_files::ImportFiles;
pub use self::instruction::{Instruction, InstructionKind};

pub mod api_request;
mod arithmetic;
/// Defined constants and ability to create more.
pub mod constants;
/// Expose feature to import external geometry files.
pub mod import_files;
/// KCVM aka KCEP instructions.
pub mod instruction;
mod memory;
pub mod sketch_types;
#[cfg(test)]
mod tests;

/// Somewhere values can be written to.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Destination {
    /// Write to main memory at the given address.
    Address(Address),
    /// Push onto the stack.
    StackPush,
    /// Extend what is already on the stack.
    StackExtend,
}

impl std::fmt::Display for Destination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Destination::Address(a) => a.fmt(f),
            Destination::StackPush => "StackPush".fmt(f),
            Destination::StackExtend => "StackExtend".fmt(f),
        }
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

/// Executing the program failed.
#[derive(Debug)]
pub struct ExecutionFailed {
    /// What error occurred.
    pub error: ExecutionError,
    /// Which instruction was being executed when the error occurred?
    pub instruction: Option<Instruction>,
    /// Which instruction number was being executed when the error occurred?
    pub instruction_index: usize,
}

/// Execute the plan.
pub async fn execute(
    mem: &mut Memory,
    plan: Vec<Instruction>,
    session: &mut Option<ModelingSession>,
) -> std::result::Result<(), ExecutionFailed> {
    let mut events = EventWriter::default();
    let mut batch_queue = ModelingBatch::default();
    let n = plan.len();
    for (i, instruction) in plan.into_iter().enumerate() {
        if let Err(e) = instruction
            .clone()
            .execute(mem, session, &mut events, &mut batch_queue)
            .await
        {
            return Err(ExecutionFailed {
                error: e,
                instruction: Some(instruction),
                instruction_index: i,
            });
        }
    }
    cleanup(session, batch_queue, &mut events, n).await?;
    Ok(())
}

async fn cleanup(
    session: &mut Option<ModelingSession>,
    batch_queue: ModelingBatch,
    events: &mut EventWriter,
    n: usize,
) -> std::result::Result<(), ExecutionFailed> {
    if batch_queue.is_empty() {
        return Ok(());
    }
    let Some(session) = session else {
        return Err(ExecutionFailed {
            error: ExecutionError::NoApiClient,
            instruction: None,
            instruction_index: n,
        });
    };
    crate::api_request::flush_batch_queue(session, batch_queue, events)
        .await
        .map_err(|e| ExecutionFailed {
            error: e,
            instruction: None,
            instruction_index: n,
        })?;
    Ok(())
}

/// Current state of execution.
pub struct ExecutionState {
    /// State of memory after executing the instruction
    pub mem: Memory,
    /// Which instruction was executed? Index into the `Vec<Instruction>` for the plan.
    pub active_instruction: usize,
    /// Which events occurred during execution of this instruction?
    pub events: Vec<Event>,
}

/// Execute the plan, returning the state at every moment of execution.
/// Also return the index of the final instruction executed.
/// This will be the last instruction if execution succeeded, but it might be earlier if
/// execution had an error and quit.
pub async fn execute_time_travel(
    mem: &mut Memory,
    plan: Vec<Instruction>,
    session: &mut Option<ModelingSession>,
) -> (Vec<ExecutionState>, usize) {
    let mut out = Vec::new();
    let mut events = EventWriter::default();
    let mut batch_queue = Default::default();
    let n = plan.len();
    for (active_instruction, instruction) in plan.into_iter().enumerate() {
        let res = instruction.execute(mem, session, &mut events, &mut batch_queue).await;

        let mut crashed = false;
        if let Err(e) = res {
            events.push(Event {
                text: e.to_string(),
                severity: events::Severity::Error,
                related_addresses: Vec::new(),
            });
            crashed = true;
        }
        let state = ExecutionState {
            mem: mem.clone(),
            active_instruction,
            events: events.drain(),
        };

        out.push(state);
        if crashed {
            return (out, active_instruction);
        }
    }
    if let Err(e) = cleanup(session, batch_queue, &mut events, n).await {
        events.push(Event {
            text: e.error.to_string(),
            severity: events::Severity::Error,
            related_addresses: Default::default(),
        });
        out.push(ExecutionState {
            mem: mem.clone(),
            active_instruction: n - 1,
            events: events.drain(),
        });
    }
    (out, n - 1)
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
    /// List index out of bounds.
    #[error("you tried to access element {index} in a list of size {count}")]
    ListIndexOutOfBounds {
        /// Number of elements in the list.
        count: usize,
        /// Index which user attempted to access.
        index: usize,
    },
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
    /// No such SketchGroup exists.
    #[error("No SketchGroup exists at index {index}")]
    NoSketchGroup {
        /// Index into the vector of SketchGroups.
        index: usize,
    },
    /// SketchGroup storage cannot have gaps.
    #[error(
        "You tried to set a SketchGroup into destination {destination} but no such index exists. The last slot available is {len}."
    )]
    SketchGroupNoGaps {
        /// Index user tried to write into.
        destination: usize,
        /// Current SketchGroup vec length.
        len: usize,
    },
    /// Invalid argument type
    #[error("An argument of the wrong type was used.")]
    BadArg {
        /// The reason why the argument is bad.
        reason: String,
    },
    /// A general execution error.
    #[error("A general execution error.")]
    General {
        /// The reason for the error.
        reason: String,
    },
}
