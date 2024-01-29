//! A KittyCAD execution plan (KCEP) is a list of
//! - KittyCAD API requests to make
//! - Values to send in API requests
//! - Values to assign from API responses
//! - Computation to perform on values
//! You can think of it as a domain-specific language for making KittyCAD API calls and using
//! the results to make other API calls.

use kittycad_execution_plan_traits::{FromMemory, ListHeader, MemoryError, NumericPrimitive, Primitive, ReadMemory};
use kittycad_modeling_cmds::{each_cmd, id::ModelingCmdId};
use kittycad_modeling_session::{RunCommandError, Session as ModelingSession};
pub use memory::{Memory, StaticMemoryInitializer};
use serde::{Deserialize, Serialize};

pub use self::address::Address;
pub use self::arithmetic::{
    operator::{BinaryOperation, Operation, UnaryOperation},
    BinaryArithmetic, UnaryArithmetic,
};

mod address;
mod arithmetic;
mod memory;
#[cfg(test)]
mod tests;

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
    /// Get the element at `index` of the list which begins at `start` into the `destination`.
    /// Push it onto the stack (does not include the element length header).
    /// Assumes the list is formatted according to [`Instruction::SetList`] documentation.
    GetElement {
        /// Starting address of the list
        start: Address,
        /// Element number
        index: Operand,
    },
    /// Set a list of elements into memory.
    /// # Format
    /// Lists have this format (each line represents a memory address starting at `start`):
    ///
    /// <number of elements>
    /// <n = size of element 0>
    /// <element 0, address 0>
    /// <...>
    /// <element 0, address n>
    /// <n = size of element 1>
    /// <element 1, address 0>
    /// <...>
    /// <element 1, address n>
    /// etc etc for each element.
    SetList {
        /// List will start at this element.
        start: Address,
        /// Each element
        elements: Vec<Vec<Primitive>>,
    },
    /// Perform arithmetic on values in memory.
    BinaryArithmetic {
        /// What to do.
        arithmetic: BinaryArithmetic,
        /// Write the output to this memory address.
        destination: Destination,
    },
    /// Perform arithmetic on a value in memory.
    UnaryArithmetic {
        /// What to do.
        arithmetic: UnaryArithmetic,
        /// Write the output to this memory address.
        destination: Destination,
    },
    /// Push this data onto the stack.
    StackPush {
        /// Data that will be pushed.
        data: Vec<Primitive>,
    },
    /// Pop data off the stack into memory.
    StackPop {
        /// If Some, the value popped will be stored at that address.
        /// If None, the value won't be stored anywhere.
        destination: Option<Address>,
    },
}

/// Somewhere values can be written to.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Destination {
    /// Write to main memory at the given address.
    Address(Address),
    /// Push onto the stack.
    StackPush,
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

/// Argument to an operation.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
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
    for step in plan.into_iter() {
        match step {
            Instruction::ApiRequest(req) => {
                if let Some(ref mut session) = session {
                    req.execute(session, mem).await?;
                } else {
                    return Err(ExecutionError::NoApiClient);
                }
            }
            Instruction::SetPrimitive { address, value } => {
                mem.set(address, value);
            }
            Instruction::SetValue { address, value_parts } => {
                value_parts.into_iter().enumerate().for_each(|(i, part)| {
                    mem.set(address.offset(i), part);
                });
            }
            Instruction::BinaryArithmetic {
                arithmetic,
                destination,
            } => {
                let out = arithmetic.calculate(mem)?;
                match destination {
                    Destination::Address(addr) => mem.set(addr, out),
                    Destination::StackPush => mem.stack.push(vec![out]),
                };
            }
            Instruction::UnaryArithmetic {
                arithmetic,
                destination,
            } => {
                let out = arithmetic.calculate(mem)?;
                match destination {
                    Destination::Address(addr) => mem.set(addr, out),
                    Destination::StackPush => mem.stack.push(vec![out]),
                };
            }
            Instruction::SetList { start, elements } => {
                // Store size of list.
                let mut curr = start;
                curr += 1;
                let n = elements.len();
                for element in elements {
                    // Store each element's size
                    mem.set(curr, element.len().into());
                    curr += 1;
                    // Then store each primitive of the element.
                    for primitive in element {
                        mem.set(curr, primitive);
                        curr += 1
                    }
                }
                mem.set(
                    start,
                    Primitive::from(ListHeader {
                        count: n,
                        size: (curr - start) - 1,
                    }),
                );
            }
            Instruction::GetElement { start, index } => {
                // Resolve the index.
                let index_primitive: Primitive = match index {
                    // Any numeric literal will do, as long as it's >= 0.
                    Operand::Literal(p) => p,
                    Operand::Reference(addr) => mem.get(&addr).ok_or(ExecutionError::MemoryEmpty { addr })?.clone(),
                    Operand::StackPop => mem.stack.pop_single()?,
                };
                let index = match index_primitive {
                    Primitive::NumericValue(NumericPrimitive::UInteger(i)) => i,
                    Primitive::NumericValue(NumericPrimitive::Integer(i)) if i >= 0 => i.try_into().unwrap(),
                    other => {
                        return Err(ExecutionError::MemoryError(MemoryError::MemoryWrongType {
                            expected: "non-negative integer",
                            actual: format!("{other:?}"),
                        }))
                    }
                };

                // Check size of the list.
                let ListHeader { count, size: _ }: ListHeader = mem.get_primitive(&start)?;
                if index >= count {
                    return Err(ExecutionError::ListIndexOutOfBounds { count, index });
                }
                // Find the given element
                let mut curr = start + 1;
                eprintln!("{}", mem.debug_table());
                eprintln!("Starting curr at {start}+1={curr}");
                for _ in 0..index {
                    let size_of_element: usize = match mem.get(&curr).ok_or(MemoryError::MemoryWrongSize)? {
                        Primitive::NumericValue(NumericPrimitive::UInteger(size)) => *size,
                        Primitive::ListHeader(ListHeader { count: _, size }) => *size,
                        other => {
                            return Err(ExecutionError::MemoryError(MemoryError::MemoryWrongType {
                                expected: "ListHeader or usize",
                                actual: format!("{other:?}"),
                            }))
                        }
                    };
                    curr += size_of_element + 1;
                }
                let size_of_element: usize = mem.get_primitive(&curr)?;
                let element = mem.get_slice(curr + 1, size_of_element)?;
                mem.stack.push(element);
            }
            Instruction::StackPush { data } => {
                mem.stack.push(data);
            }
            Instruction::StackPop { destination } => {
                let data = mem.stack.pop()?;
                let Some(destination) = destination else { continue };
                for (i, data_part) in data.into_iter().enumerate() {
                    mem.set(destination + i, data_part);
                }
            }
        }
    }
    Ok(())
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
}
