//! Calculate which instructions each API request depends on.

use std::collections::HashMap;

use kittycad_execution_plan_traits::Address;

use crate::{Destination, Instruction, Operand};

fn writes_to_dst(
    destination: &Destination,
    instr_id: usize,
    wrote_to_address: &mut HashMap<Address, usize>,
    wrote_to_stack: &mut Vec<Vec<usize>>,
) {
    match destination {
        crate::Destination::Address(a) => {
            wrote_to_address.insert(*a, instr_id);
        }
        crate::Destination::StackPush => wrote_to_stack.push(vec![instr_id]),
        crate::Destination::StackExtend => {
            if let Some(x) = wrote_to_stack.last_mut() {
                x.push(instr_id);
            } else {
                wrote_to_stack.push(vec![instr_id]);
            }
        }
    }
}

fn reads_from_operand(
    op: &Operand,
    instr_id: usize,
    wrote_to_address: &mut HashMap<Address, usize>,
    wrote_to_stack: &mut Vec<Vec<usize>>,
    depends_on: &mut HashMap<usize, Vec<usize>>,
) {
    match op {
        Operand::Literal(_) => {}
        Operand::Reference(a) => {
            depends_on
                .entry(instr_id)
                .or_default()
                .push(*wrote_to_address.get(a).unwrap());
        }
        Operand::StackPop => {
            depends_on
                .entry(instr_id)
                .or_default()
                .extend(wrote_to_stack.pop().unwrap());
        }
    }
}

fn reads_from(
    address: Address,
    instr_id: usize,
    wrote_to_address: &mut HashMap<Address, usize>,
    depends_on: &mut HashMap<usize, Vec<usize>>,
) {
    depends_on
        .entry(instr_id)
        .or_default()
        .push(*wrote_to_address.get(&address).unwrap());
}

/// For each instruction, calculate which previous instructions it requires.
pub fn calculate(instructions: Vec<Instruction>) -> HashMap<usize, Vec<usize>> {
    let mut wrote_to_stack: Vec<Vec<usize>> = Vec::new();
    let mut wrote_to_address: HashMap<Address, usize> = HashMap::new();
    // (K,V) indicates that K depends on V.
    let mut depends_on: HashMap<usize, Vec<usize>> = HashMap::new();

    for (instr_id, instruction) in instructions.iter().enumerate() {
        match instruction {
            Instruction::ApiRequest(_) => todo!(),
            Instruction::SetPrimitive { address, value: _ } => {
                wrote_to_address.insert(*address, instr_id);
            }
            Instruction::SetValue { address, value_parts } => {
                for i in 0..value_parts.len() {
                    wrote_to_address.insert(*address + i, instr_id);
                }
            }
            Instruction::AddrOfMember { start, member } => {
                wrote_to_stack.push(vec![instr_id]);
                reads_from_operand(
                    start,
                    instr_id,
                    &mut wrote_to_address,
                    &mut wrote_to_stack,
                    &mut depends_on,
                );
                reads_from_operand(
                    member,
                    instr_id,
                    &mut wrote_to_address,
                    &mut wrote_to_stack,
                    &mut depends_on,
                );
            }
            Instruction::SetList { start, elements: _ } => {
                reads_from(*start, instr_id, &mut wrote_to_address, &mut depends_on);
            }
            Instruction::BinaryArithmetic {
                arithmetic,
                destination,
            } => {
                reads_from_operand(
                    &arithmetic.operand0,
                    instr_id,
                    &mut wrote_to_address,
                    &mut wrote_to_stack,
                    &mut depends_on,
                );
                reads_from_operand(
                    &arithmetic.operand1,
                    instr_id,
                    &mut wrote_to_address,
                    &mut wrote_to_stack,
                    &mut depends_on,
                );
                writes_to_dst(destination, instr_id, &mut wrote_to_address, &mut wrote_to_stack);
            }
            Instruction::UnaryArithmetic {
                arithmetic,
                destination,
            } => {
                reads_from_operand(
                    &arithmetic.operand,
                    instr_id,
                    &mut wrote_to_address,
                    &mut wrote_to_stack,
                    &mut depends_on,
                );
                writes_to_dst(destination, instr_id, &mut wrote_to_address, &mut wrote_to_stack);
            }
            Instruction::StackPush { data: _ } => wrote_to_stack.push(vec![instr_id]),
            Instruction::StackPop { destination } => {
                let deps = wrote_to_stack.pop().unwrap();
                depends_on.insert(instr_id, deps);
                if let Some(dst) = destination {
                    writes_to_dst(dst, instr_id, &mut wrote_to_address, &mut wrote_to_stack);
                }
            }
            Instruction::StackExtend { data } => todo!(),
            Instruction::Copy {
                source,
                length: _,
                destination,
            } => {
                reads_from(*source, instr_id, &mut wrote_to_address, &mut depends_on);
                writes_to_dst(destination, instr_id, &mut wrote_to_address, &mut wrote_to_stack);
            }
            Instruction::CopyLen {
                source_range,
                destination_range,
            } => {
                reads_from_operand(
                    source_range,
                    instr_id,
                    &mut wrote_to_address,
                    &mut wrote_to_stack,
                    &mut depends_on,
                );
                writes_to_operand(destination_range, instr_id, &mut wrote_to_address, &mut wrote_to_stack);
            }
            Instruction::SketchGroupSet {
                sketch_group,
                destination,
            } => todo!(),
            Instruction::SketchGroupAddSegment {
                segment,
                source,
                destination,
            } => todo!(),
            Instruction::SketchGroupSetBasePath { source, from, to, name } => todo!(),
            Instruction::SketchGroupCopyFrom {
                source,
                offset,
                length,
                destination,
            } => todo!(),
            Instruction::SketchGroupGetLastPoint { source, destination } => todo!(),
            Instruction::NoOp { comment } => todo!(),
        }
    }
    depends_on
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let instructions_json = std::fs::read_to_string("../execution-plan-debugger/test_input.json").unwrap();
        let instructions: Vec<Instruction> = serde_json::from_str(&instructions_json).unwrap();
        calculate(instructions);
    }
}
