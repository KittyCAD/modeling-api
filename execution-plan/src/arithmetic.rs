use kittycad_execution_plan_traits::{NumericPrimitive, Primitive};
use serde::{Deserialize, Serialize};

use self::operator::{BinaryOperation, UnaryOperation};
use crate::{ExecutionError, Memory, Operand};

pub mod operator;

/// Instruction to perform arithmetic on values in memory.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct BinaryArithmetic {
    /// Apply this operation
    pub operation: BinaryOperation,
    /// First operand for the operation
    pub operand0: Operand,
    /// Second operand for the operation
    pub operand1: Operand,
}

/// Instruction to perform arithmetic on values in memory.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct UnaryArithmetic {
    /// Apply this operation
    pub operation: UnaryOperation,
    /// Operand for the operation
    pub operand: Operand,
}
impl UnaryArithmetic {
    pub(crate) fn calculate(self, mem: &mut Memory) -> Result<Primitive, ExecutionError> {
        let val = self.operand.eval(mem)?.clone();
        match self.operation {
            UnaryOperation::Not => {
                if let Primitive::Bool(b) = val {
                    Ok(Primitive::Bool(!b))
                } else {
                    Err(ExecutionError::CannotApplyOperation {
                        op: self.operation.into(),
                        operands: vec![val],
                    })
                }
            }
            UnaryOperation::Neg => match val {
                Primitive::NumericValue(NumericPrimitive::Float(x)) => {
                    Ok(Primitive::NumericValue(NumericPrimitive::Float(-x)))
                }
                Primitive::NumericValue(NumericPrimitive::Integer(x)) => {
                    Ok(Primitive::NumericValue(NumericPrimitive::Integer(-x)))
                }
                _ => Err(ExecutionError::CannotApplyOperation {
                    op: self.operation.into(),
                    operands: vec![val],
                }),
            },
        }
    }
}

macro_rules! arithmetic_body {
    ($arith:ident, $mem:ident, $method:ident) => {
        match ($arith.operand0.eval($mem)?.clone(), $arith.operand1.eval($mem)?.clone()) {
            // If both operands are numeric, then do the arithmetic operation.
            (Primitive::NumericValue(x), Primitive::NumericValue(y)) => {
                let num = match (x, y) {
                    (NumericPrimitive::UInteger(x), NumericPrimitive::UInteger(y)) => {
                        NumericPrimitive::UInteger(x.$method(y))
                    }
                    (NumericPrimitive::UInteger(x), NumericPrimitive::Float(y)) => {
                        NumericPrimitive::Float((x as f64).$method(y))
                    }
                    (NumericPrimitive::Float(x), NumericPrimitive::UInteger(y)) => {
                        NumericPrimitive::Float(x.$method(y as f64))
                    }
                    (NumericPrimitive::Float(x), NumericPrimitive::Float(y)) => NumericPrimitive::Float(x.$method(y)),
                    (NumericPrimitive::Integer(x), NumericPrimitive::Integer(y)) => {
                        NumericPrimitive::Integer(x.$method(y))
                    }
                    (NumericPrimitive::Integer(x), NumericPrimitive::Float(y)) => {
                        NumericPrimitive::Float((x as f64).$method(y))
                    }
                    (NumericPrimitive::Float(x), NumericPrimitive::Integer(y)) => {
                        NumericPrimitive::Float(x.$method(y as f64))
                    }
                    (NumericPrimitive::Integer(x), NumericPrimitive::UInteger(y)) => {
                        NumericPrimitive::Integer(x.$method(y as i64))
                    }
                    (NumericPrimitive::UInteger(x), NumericPrimitive::Integer(y)) => {
                        NumericPrimitive::Integer((x as i64).$method(y))
                    }
                };
                Ok(Primitive::NumericValue(num))
            }
            // This operation can only be done on numeric types.
            _ => Err(ExecutionError::CannotApplyOperation {
                op: $arith.operation.into(),
                operands: vec![
                    $arith.operand0.eval($mem)?.clone().to_owned(),
                    $arith.operand1.eval($mem)?.clone().to_owned(),
                ],
            }),
        }
    };
}
impl BinaryArithmetic {
    /// Calculate the the arithmetic equation.
    /// May read values from the given memory.
    pub fn calculate(self, mem: &mut Memory) -> Result<Primitive, ExecutionError> {
        use std::ops::{Add, Div, Mul, Sub};
        match self.operation {
            BinaryOperation::Add => {
                arithmetic_body!(self, mem, add)
            }
            BinaryOperation::Mul => {
                arithmetic_body!(self, mem, mul)
            }
            BinaryOperation::Sub => {
                arithmetic_body!(self, mem, sub)
            }
            BinaryOperation::Div => {
                arithmetic_body!(self, mem, div)
            }
            BinaryOperation::Mod => todo!(),
            BinaryOperation::Pow => todo!(),
        }
    }
}
