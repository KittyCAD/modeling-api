use kittycad_execution_plan_traits::{NumericPrimitive, Primitive};
use serde::{Deserialize, Serialize};

use self::operator::{BinaryOperation, UnaryOperation};
use crate::{events::EventWriter, ExecutionError, Memory, Operand};

pub mod operator;

/// Instruction to perform arithmetic on values in memory.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct BinaryArithmetic {
    /// Apply this operation
    pub operation: BinaryOperation,
    /// First operand for the operation
    pub operand0: Operand,
    /// Second operand for the operation
    pub operand1: Operand,
}

/// Instruction to perform arithmetic on values in memory.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
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

trait Power {
    fn power(self, other: Self) -> Self;
}

macro_rules! power_float_impl {
    ($($t:ty)*) => ($(
        impl Power for $t {
            fn power(self, other: $t) -> $t { self.powf(other) }
        }

    )*)
}
power_float_impl! { f32 f64 }

macro_rules! power_int_impl {
    ($($t:ty)*) => ($(
        impl Power for $t {
            fn power(self, other: $t) -> $t { self.overflowing_pow(other as u32).0 }
        }

    )*)
}
power_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

macro_rules! arithmetic_body {
    ($arith:ident, $mem:ident, $method:ident, $events:ident) => {{
        $events.push(crate::events::Event::new(
            "Evaluating left operand".to_owned(),
            crate::events::Severity::Debug,
        ));
        let l = $arith.operand0.eval($mem)?.clone();
        $events.push({
            let mut evt = crate::events::Event::new(format!("Left operand is {l:?}"), crate::events::Severity::Info);
            if let Operand::Reference(a) = $arith.operand0 {
                evt.related_address = Some(a);
            }
            evt
        });
        $events.push(crate::events::Event::new(
            "Evaluating right operand".to_owned(),
            crate::events::Severity::Debug,
        ));
        let r = $arith.operand1.eval($mem)?.clone();
        $events.push({
            let mut evt = crate::events::Event::new(format!("Right operand is {r:?}"), crate::events::Severity::Info);
            if let Operand::Reference(a) = $arith.operand1 {
                evt.related_address = Some(a);
            }
            evt
        });
        match (l, r) {
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
                let prim = Primitive::NumericValue(num);
                $events.push(crate::events::Event::new(
                    format!("Output is {prim:?}"),
                    crate::events::Severity::Info,
                ));
                Ok(prim)
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
    }};
}
impl BinaryArithmetic {
    /// Calculate the the arithmetic equation.
    /// May read values from the given memory.
    pub fn calculate(self, mem: &mut Memory, events: &mut EventWriter) -> Result<Primitive, ExecutionError> {
        use std::ops::{Add, Div, Mul, Sub, Rem};
        match self.operation {
            BinaryOperation::Add => {
                arithmetic_body!(self, mem, add, events)
            }
            BinaryOperation::Mul => {
                arithmetic_body!(self, mem, mul, events)
            }
            BinaryOperation::Sub => {
                arithmetic_body!(self, mem, sub, events)
            }
            BinaryOperation::Div => {
                arithmetic_body!(self, mem, div, events)
            }
            BinaryOperation::Mod => {
                arithmetic_body!(self, mem, rem, events)
            }
            BinaryOperation::Pow => {
                arithmetic_body!(self, mem, power, events)
            }
        }
    }
}
