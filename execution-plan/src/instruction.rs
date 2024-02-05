use kittycad_execution_plan_traits::{ListHeader, MemoryError, NumericPrimitive, ObjectHeader, Primitive, ReadMemory};
use serde::{Deserialize, Serialize};

use crate::{
    events::{Event, EventWriter, Severity},
    Address, ApiRequest, BinaryArithmetic, Destination, ExecutionError, Memory, Operand, Result, UnaryArithmetic,
};

/// One step of the execution plan.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
    /// Find an element/property of an array/object.
    /// Push the element/property's address onto the stack.
    /// Assumes the object/list is formatted according to [`Instruction::SetList`] documentation.
    AddrOfMember {
        /// Starting address of the array/object.
        start: Address,
        /// Element index or property name.
        member: Operand,
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
        events: &mut EventWriter,
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
                let out = arithmetic.calculate(mem, events)?;
                match destination {
                    Destination::Address(addr) => {
                        events.push(Event {
                            text: format!("Writing output to address {addr}"),
                            severity: crate::events::Severity::Info,
                            related_address: Some(addr),
                        });
                        mem.set(addr, out);
                    }
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
            Instruction::AddrOfMember { start, member } => {
                // Read the member.
                let member_primitive: Primitive = match member {
                    Operand::Literal(p) => p,
                    Operand::Reference(addr) => mem.get(&addr).ok_or(ExecutionError::MemoryEmpty { addr })?.clone(),
                    Operand::StackPop => mem.stack.pop_single()?,
                };
                events.push(Event {
                    text: format!("Property is '{member_primitive:?}'"),
                    severity: Severity::Debug,
                    related_address: None,
                });

                // Read the structure.
                let structure = mem
                    .get(&start)
                    .cloned()
                    .ok_or(ExecutionError::MemoryEmpty { addr: start })?;
                events.push(Event {
                    text: format!("Looking up property of '{structure:?}'"),
                    severity: Severity::Debug,
                    related_address: Some(start),
                });

                // Look up the member in this structure. What number member is it?
                let (index, member_display) = match structure {
                    // Structure is an array
                    Primitive::ListHeader(ListHeader { count, size: _ }) => match member_primitive {
                        Primitive::NumericValue(NumericPrimitive::UInteger(i)) => {
                            // Bounds check
                            if i < count {
                                events.push(Event {
                                    text: format!("Property is index {i}"),
                                    severity: Severity::Info,
                                    related_address: None,
                                });
                                (i, i.to_string())
                            } else {
                                return Err(ExecutionError::ListIndexOutOfBounds { count, index: i });
                            }
                        }
                        other_index => {
                            return Err(ExecutionError::MemoryError(MemoryError::MemoryWrongType {
                                expected: "uint",
                                actual: format!("{other_index:?}"),
                            }));
                        }
                    },
                    // Structure is an object
                    Primitive::ObjectHeader(ObjectHeader { properties, size: _ }) => match member_primitive {
                        Primitive::String(s) => {
                            // Property check
                            if let Some(i) = properties.iter().position(|prop| prop == &s) {
                                events.push(Event {
                                    text: format!("Property is index {i}"),
                                    severity: Severity::Info,
                                    related_address: None,
                                });
                                (i, s.clone())
                            } else {
                                return Err(ExecutionError::UndefinedProperty {
                                    property: s,
                                    address: start,
                                });
                            }
                        }
                        other_index => {
                            return Err(ExecutionError::MemoryError(MemoryError::MemoryWrongType {
                                expected: "uint",
                                actual: format!("{other_index:?}"),
                            }))
                        }
                    },
                    other_structure => {
                        return Err(ExecutionError::MemoryError(MemoryError::MemoryWrongType {
                            expected: "list or object header",
                            actual: format!("{other_structure:?}"),
                        }))
                    }
                };

                // Find the address of the given member.
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
                events.push(Event {
                    text: format!("Member '{member_display}' begins at addr {curr}"),
                    severity: crate::events::Severity::Info,
                    related_address: Some(curr),
                });
                mem.stack.push(vec![curr.0.into()]);
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
