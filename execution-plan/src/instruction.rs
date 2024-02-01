use kittycad_execution_plan_traits::{ListHeader, MemoryError, NumericPrimitive, ObjectHeader, Primitive, ReadMemory};
use serde::{Deserialize, Serialize};

use crate::{
    Address, ApiRequest, BinaryArithmetic, Destination, ExecutionError, Memory, Operand, Result, UnaryArithmetic,
};

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
    /// Get the element at `index` of the list which begins at `start` into the `destination`.
    /// Push it onto the stack (does not include the element length header).
    /// Objects are laid out like lists, but with different header.
    GetProperty {
        /// Starting address of the object.
        start: Address,
        /// Which property to retrieve
        property: Operand,
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

impl Instruction {
    /// Execute the instruction
    pub async fn execute(
        self,
        mem: &mut Memory,
        session: Option<&mut kittycad_modeling_session::Session>,
    ) -> Result<()> {
        match self {
            Instruction::ApiRequest(req) => {
                if let Some(session) = session {
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
                for _ in 0..index {
                    let size_of_element: usize = match mem.get(&curr).ok_or(MemoryError::MemoryWrongSize)? {
                        Primitive::NumericValue(NumericPrimitive::UInteger(size)) => *size,
                        Primitive::ListHeader(ListHeader { count: _, size }) => *size,
                        Primitive::ObjectHeader(ObjectHeader { properties: _, size }) => *size,
                        other => {
                            return Err(ExecutionError::MemoryError(MemoryError::MemoryWrongType {
                                expected: "ListHeader, ObjectHeader, or usize",
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
            Instruction::GetProperty { start, property } => {
                // Resolve the index.
                let property_primitive: Primitive = match property {
                    // Any numeric literal will do, as long as it's >= 0.
                    Operand::Literal(p) => p,
                    Operand::Reference(addr) => mem.get(&addr).ok_or(ExecutionError::MemoryEmpty { addr })?.clone(),
                    Operand::StackPop => mem.stack.pop_single()?,
                };
                let property = match property_primitive {
                    Primitive::String(p) => p,
                    other => {
                        return Err(ExecutionError::MemoryError(MemoryError::MemoryWrongType {
                            expected: "String",
                            actual: format!("{other:?}"),
                        }))
                    }
                };

                // Check size of the list.
                let ObjectHeader { properties, size: _ }: ObjectHeader = mem.get_primitive(&start)?;
                let index =
                    properties
                        .iter()
                        .position(|prop| prop == &property)
                        .ok_or(ExecutionError::UndefinedProperty {
                            property,
                            address: start,
                        })?;
                // Find the given element
                let mut curr = start + 1;
                for _ in 0..index {
                    let size_of_element = mem.get_size(&curr)?;
                    curr += size_of_element + 1;
                }
                let size_of_element: usize = mem.get_size(&curr)?;
                let element = mem.get_slice(curr + 1, size_of_element)?;
                mem.stack.push(element);
            }
            Instruction::StackPush { data } => {
                mem.stack.push(data);
            }
            Instruction::StackPop { destination } => {
                let data = mem.stack.pop()?;
                let Some(destination) = destination else { return Ok(()) };
                for (i, data_part) in data.into_iter().enumerate() {
                    mem.set(destination + i, data_part);
                }
            }
        }
        Ok(())
    }
}
