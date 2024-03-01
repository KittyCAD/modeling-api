use kittycad_execution_plan_traits::{
    InMemory, ListHeader, MemoryError, NumericPrimitive, ObjectHeader, Primitive, ReadMemory, Value,
};
use kittycad_modeling_cmds::{shared::Point2d, websocket::ModelingBatch};
use serde::{Deserialize, Serialize};

use crate::{
    events::{Event, EventWriter, Severity},
    sketch_types::{self},
    Address, ApiRequest, BinaryArithmetic, Destination, ExecutionError, ImportFiles, Memory, Operand, Result,
    UnaryArithmetic,
};

/// One step of the execution plan.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Instruction {
    /// Call the KittyCAD API.
    ApiRequest(ApiRequest),
    /// Import a geometry file.
    ImportFiles(ImportFiles),
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
        start: Operand,
        /// Element index or property name.
        member: Operand,
    },
    /// Set a list of elements into memory.
    /// # Format
    /// Lists have this format (each line represents a memory address starting at `start`):
    ///
    /// ```nocode
    /// <number of elements>
    /// <n = size of element 0>
    /// <element 0, address 0>
    /// <...>
    /// <element 0, address n>
    /// <n = size of element 1>
    /// <element 1, address 0>
    /// <...>
    /// <element 1, address n>
    /// ```
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
        /// If Some, the value popped will be stored at the destination.
        /// If None, the value won't be stored anywhere.
        destination: Option<Destination>,
    },
    /// Add the given primitives to whatever is on top of the stack.
    /// If the stack is empty, runtime error.
    StackExtend {
        /// Extend whatever is on top of the stack with this new data.
        data: Vec<Primitive>,
    },
    /// Copy from one address to the other.
    Copy {
        /// Copy from here.
        source: Address,
        /// How many addresses to copy.
        length: usize,
        /// Copy to here.
        destination: Destination,
    },
    /// Copy data from a range of addresses, into another range of addresses.
    /// The first address in the source range is the length (how many addresses to copy).
    /// If that address contains a uint, that uint is the length.
    /// If that address contains a List/Object header, the `size` field is the length.
    /// Source range is evaluated before destination range (this is only relevant if both source
    /// and destination come from the stack).
    CopyLen {
        /// Start copying from this address.
        source_range: Operand,
        /// Start copying into this address.
        destination_range: Operand,
    },
    /// Write the SketchGroup to its special storage.
    SketchGroupSet {
        /// What to write.
        sketch_group: sketch_types::SketchGroup,
        /// Index into the SketchGroup storage vec.
        destination: usize,
    },
    /// Add a path to a SketchGroup.
    SketchGroupAddSegment {
        /// Address of a PathSegment which will be added to the SketchGroup.
        segment: InMemory,
        /// Where the SketchGroup to modify begins.
        /// This is an index into the `SketchGroup` storage of the memory.
        source: usize,
        /// Where the modified SketchGroup should be written to.
        destination: usize,
    },
    /// Set the base path of a SketchGroup.
    SketchGroupSetBasePath {
        /// Where the SketchGroup to modify begins.
        /// This is an index into the `SketchGroup` storage of the memory.
        source: usize,
        /// Where the base path starts.
        from: InMemory,
        /// Where the base path ends.
        to: InMemory,
        /// The name of the base path.
        name: Option<InMemory>,
    },
    /// Copy data from a SketchGroup.
    SketchGroupCopyFrom {
        /// Index into the SketchGroup array.
        source: usize,
        /// Which offset into the SketchGroup's Vec<Primitive> should copying start at?
        offset: usize,
        /// How many primitives should be copied?
        length: usize,
        /// Where to copy them to.
        destination: Destination,
    },
    /// Get the `to` end of the last path segment, i.e. the point from which the next segment will start.
    SketchGroupGetLastPoint {
        /// Which SketchGroup to examine.
        source: usize,
        /// Where to copy the data.
        destination: Destination,
    },
    /// Does nothing. Used for debugging.
    NoOp {
        /// Debug message.
        comment: String,
    },
}

impl Instruction {
    /// Execute the instruction
    pub async fn execute(
        self,
        mem: &mut Memory,
        session: &mut Option<kittycad_modeling_session::Session>,
        events: &mut EventWriter,
        batch_queue: &mut ModelingBatch,
    ) -> Result<()> {
        match self {
            Instruction::NoOp { comment: _ } => {}
            Instruction::ApiRequest(req) => {
                if let Some(session) = session {
                    req.execute(session, mem, events, batch_queue).await?;
                } else {
                    return Err(ExecutionError::NoApiClient);
                }
            }
            Instruction::ImportFiles(req) => {
                req.execute(mem).await?;
            }
            Instruction::SetPrimitive { address, value } => {
                events.push(Event {
                    text: format!("Writing output to address {address}"),
                    severity: crate::events::Severity::Info,
                    related_addresses: vec![address],
                });
                mem.set(address, value);
            }
            Instruction::Copy {
                source,
                length,
                destination,
            } => {
                let sources: Vec<_> = (0..length).map(|i| source + i).collect();
                // Read the value
                events.push(Event {
                    text: "Reading value".to_owned(),
                    severity: Severity::Debug,
                    related_addresses: sources.clone(),
                });

                let data = sources
                    .iter()
                    .map(|i| mem.get(i).cloned().ok_or(ExecutionError::MemoryEmpty { addr: source }))
                    .collect::<Result<Vec<_>>>()?;
                write_to_dst(data, destination, mem, events)?;
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
                            related_addresses: vec![addr],
                        });
                        mem.set(addr, out);
                    }
                    Destination::StackPush => {
                        mem.stack.push(vec![out]);
                    }
                    Destination::StackExtend => {
                        mem.stack.extend(vec![out])?;
                    }
                };
            }
            Instruction::UnaryArithmetic {
                arithmetic,
                destination,
            } => {
                let out = arithmetic.calculate(mem, events)?;
                match destination {
                    Destination::Address(addr) => mem.set(addr, out),
                    Destination::StackPush => mem.stack.push(vec![out]),
                    Destination::StackExtend => mem.stack.extend(vec![out])?,
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
                    related_addresses: Vec::new(),
                });

                // Read the structure.
                events.push(Event {
                    text: format!("Resolving start address {start:?}"),
                    severity: Severity::Debug,
                    related_addresses: Vec::new(),
                });
                let start_address = match start {
                    Operand::Literal(Primitive::Address(a)) => a,
                    Operand::Literal(other) => {
                        return Err(ExecutionError::MemoryError(MemoryError::MemoryWrongType {
                            expected: "address",
                            actual: format!("{other:?}"),
                        }))
                    }
                    Operand::Reference(addr) => mem.get_primitive(&addr)?,
                    Operand::StackPop => {
                        let data = mem.stack.pop_single()?;
                        data.try_into()?
                    }
                };
                events.push(Event {
                    text: "Resolved start address".to_owned(),
                    severity: Severity::Debug,
                    related_addresses: vec![start_address],
                });
                let structure = mem
                    .get(&start_address)
                    .cloned()
                    .ok_or(ExecutionError::MemoryEmpty { addr: start_address })?;

                // Look up the member in this structure. What number member is it?
                let (index, member_display) = match structure {
                    // Structure is an array
                    Primitive::ListHeader(ListHeader { count, size: _ }) => match member_primitive {
                        Primitive::NumericValue(NumericPrimitive::Integer(i)) if i >= 0 => {
                            let i = i as usize;
                            // Bounds check
                            if i < count {
                                events.push(Event {
                                    text: format!("Property is index {i}"),
                                    severity: Severity::Info,
                                    related_addresses: Vec::new(),
                                });
                                (i, i.to_string())
                            } else {
                                return Err(ExecutionError::ListIndexOutOfBounds { count, index: i });
                            }
                        }
                        Primitive::NumericValue(NumericPrimitive::UInteger(i)) => {
                            // Bounds check
                            if i < count {
                                events.push(Event {
                                    text: format!("Property is index {i}"),
                                    severity: Severity::Info,
                                    related_addresses: Vec::new(),
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
                                    related_addresses: Vec::new(),
                                });
                                (i, s.clone())
                            } else {
                                return Err(ExecutionError::UndefinedProperty {
                                    property: s,
                                    address: start_address,
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
                let mut curr = start_address + 1;
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
                    related_addresses: vec![curr],
                });
                // Push the member onto the stack.
                // This first address will be its length.
                // The length is followed by that many addresses worth of data.
                mem.stack.push(vec![Primitive::Address(curr)]);
            }
            Instruction::StackPush { data } => {
                mem.stack.push(data);
            }
            Instruction::StackExtend { data } => {
                mem.stack.extend(data)?;
            }
            Instruction::StackPop { destination } => {
                let data = mem.stack.pop()?;
                let Some(destination) = destination else { return Ok(()) };
                write_to_dst(data, destination, mem, events)?;
            }
            Instruction::CopyLen {
                source_range,
                destination_range,
            } => {
                let src_addr = match source_range.eval(mem)? {
                    Primitive::Address(a) => a,
                    other => {
                        return Err(ExecutionError::MemoryError(MemoryError::MemoryWrongType {
                            expected: "address",
                            actual: format!("{other:?}"),
                        }))
                    }
                };
                let dst_addr = match destination_range.eval(mem)? {
                    Primitive::Address(a) => a,
                    other => {
                        return Err(ExecutionError::MemoryError(MemoryError::MemoryWrongType {
                            expected: "address",
                            actual: format!("{other:?}"),
                        }))
                    }
                };

                let len = match mem
                    .get(&src_addr)
                    .ok_or(ExecutionError::MemoryEmpty { addr: src_addr })?
                {
                    Primitive::NumericValue(NumericPrimitive::UInteger(n)) => n,
                    Primitive::ObjectHeader(ObjectHeader { size, .. }) => size,
                    Primitive::ListHeader(ListHeader { size, .. }) => size,
                    other => {
                        return Err(ExecutionError::MemoryError(MemoryError::MemoryWrongType {
                            expected: "uint or obj/list header",
                            actual: format!("{other:?}"),
                        }))
                    }
                };
                for i in 0..*len {
                    let src = src_addr + i + 1;
                    let dst = dst_addr + i;
                    let val = mem.get(&src).ok_or(ExecutionError::MemoryEmpty { addr: src })?;
                    mem.set(dst, val.clone());
                }
            }
            Instruction::SketchGroupSet {
                sketch_group,
                destination,
            } => {
                mem.sketch_group_set(sketch_group, destination)?;
            }
            Instruction::SketchGroupSetBasePath { source, from, to, name } => {
                let mut sg = mem
                    .sketch_groups
                    .get(source)
                    .ok_or(ExecutionError::NoSketchGroup { index: source })?
                    .clone();
                let from: Point2d<f64> = mem.get_in_memory(from, "from", events)?.0;
                let to: Point2d<f64> = mem.get_in_memory(to, "to", events)?.0;
                let name: String = match name {
                    Some(name) => mem.get_in_memory(name, "name", events)?.0,
                    None => String::new(),
                };
                let base_path = sketch_types::BasePath { from, to, name };
                sg.path_first = base_path;
                mem.sketch_group_set(sg, source)?;
            }
            Instruction::SketchGroupAddSegment {
                segment,
                source,
                destination,
            } => {
                let mut sg = mem
                    .sketch_groups
                    .get(source)
                    .ok_or(ExecutionError::NoSketchGroup { index: source })?
                    .clone();
                let (segment, _count) = mem.get_in_memory(segment, "segment", events)?;
                sg.path_rest.push(segment);
                mem.sketch_group_set(sg, destination)?;
            }
            Instruction::SketchGroupGetLastPoint { source, destination } => {
                let sg = mem
                    .sketch_groups
                    .get(source)
                    .ok_or(ExecutionError::NoSketchGroup { index: source })?
                    .clone();
                let p = sg.last_point();
                write_to_dst(p.into_parts(), destination, mem, events)?;
            }
            Instruction::SketchGroupCopyFrom {
                source,
                offset,
                length,
                destination,
            } => {
                let sg = mem
                    .sketch_groups
                    .get(source)
                    .ok_or(ExecutionError::NoSketchGroup { index: source })?
                    .clone()
                    .into_parts();
                let data = sg.into_iter().skip(offset).take(length).collect();
                write_to_dst(data, destination, mem, events)?;
            }
        }
        Ok(())
    }
}

fn write_to_dst(
    data: Vec<Primitive>,
    destination: Destination,
    mem: &mut Memory,
    events: &mut EventWriter,
) -> std::result::Result<(), MemoryError> {
    match destination {
        Destination::Address(dst) => {
            events.push(Event {
                text: "Writing value".to_owned(),
                severity: Severity::Debug,
                related_addresses: (0..data.len()).map(|i| dst + i).collect(),
            });
            for (i, v) in data.into_iter().enumerate() {
                mem.set(dst + i, v);
            }
            Ok(())
        }
        Destination::StackPush => {
            mem.stack.push(data);
            Ok(())
        }
        Destination::StackExtend => mem.stack.extend(data),
    }
}
