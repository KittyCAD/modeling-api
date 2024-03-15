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

macro_rules! binary_arithmetic_body {
    ($arith:ident, $mem:ident, $method:ident, $events:ident) => {{
        $events.push(crate::events::Event::new(
            "Evaluating left operand".to_owned(),
            crate::events::Severity::Debug,
        ));
        let l = $arith.operand0.eval($mem)?.clone();
        $events.push({
            let mut evt = crate::events::Event::new(format!("Left operand is {l:?}"), crate::events::Severity::Info);
            if let Operand::Reference(a) = $arith.operand0 {
                evt.related_addresses = vec![a];
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
                evt.related_addresses = vec![a];
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

macro_rules! execution_error_if_match_otherwise {
 ([], $b:ident, $err:expr, $otherwise:expr) => { $otherwise };
 ([u32$(, $as:ident)*], u32, $err:expr, $otherwise:expr) => {
   $err
 };
 ([i64$(, $as:ident)*], i64, $err:expr, $otherwise:expr) => {
   $err
 };
 ([f64$(, $as:ident)*], f64, $err:expr, $otherwise:expr) => {
   $err
 };
 ([$a:ident$(, $as:ident)*], $b:ident, $err:expr, $otherwise:expr) => {
   execution_error_if_match_otherwise!([$($as)*], $b, $err, $otherwise)
 };
}

macro_rules! unary_arithmetic_body {
    ($arith:ident, $mem:ident, $events:ident, $op:ident, $($types:tt)*) => {{
        $events.push(crate::events::Event::new(
            "Evaluating operand".to_owned(),
            crate::events::Severity::Debug,
        ));
        let operand = $arith.operand.eval($mem)?.clone();
        $events.push({
            let mut evt = crate::events::Event::new(format!("Operand is {operand:?}"), crate::events::Severity::Info);
            if let Operand::Reference(a) = $arith.operand {
                evt.related_addresses = vec![a];
            }
            evt
        });
        match operand {
            // If both operands are numeric, then do the arithmetic operation.
            Primitive::NumericValue(wrapped_x) => {
                let num = match wrapped_x {
                    // Looks like the compiler doesn't know x is actually used.
                    NumericPrimitive::UInteger(_x) => {
                      execution_error_if_match_otherwise!(
                        $($types)*, u32,
                        Err(ExecutionError::CannotApplyOperation {
                          op: $arith.operation.into(),
                          operands: vec![ $arith.operand.eval($mem)?.clone().to_owned(), ]
                        })?,
                        NumericPrimitive::Integer((_x as i64).$op())
                      )
                    }
                    NumericPrimitive::Float(_x) => {
                      execution_error_if_match_otherwise!(
                        $($types)*, f64,
                        Err(ExecutionError::CannotApplyOperation {
                          op: $arith.operation.into(),
                          operands: vec![ $arith.operand.eval($mem)?.clone().to_owned(), ]
                        })?,
                        NumericPrimitive::Float(_x.$op())
                      )
                    }
                    NumericPrimitive::Integer(_x) => {
                      execution_error_if_match_otherwise!(
                        $($types)*, i64,
                        Err(ExecutionError::CannotApplyOperation {
                          op: $arith.operation.into(),
                          operands: vec![ $arith.operand.eval($mem)?.clone().to_owned(), ]
                        })?,
                        NumericPrimitive::Integer(_x.$op())
                      )
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
                    $arith.operand.eval($mem)?.clone().to_owned(),
                ],
            }),
        }
    }};
}
impl UnaryArithmetic {
    /// Calculate unary operations
    pub fn calculate(self, mem: &mut Memory, events: &mut EventWriter) -> Result<Primitive, ExecutionError> {
        use std::ops::{Neg, Not};
        match self.operation {
            UnaryOperation::Not => {
                unary_arithmetic_body!(self, mem, events, not, [f64])
            }
            UnaryOperation::Neg => {
                unary_arithmetic_body!(self, mem, events, neg, [])
            }
            UnaryOperation::Abs => {
                unary_arithmetic_body!(self, mem, events, abs, [])
            }
            UnaryOperation::Acos => {
                unary_arithmetic_body!(self, mem, events, acos, [i64, u32])
            }
            UnaryOperation::Asin => {
                unary_arithmetic_body!(self, mem, events, asin, [i64, u32])
            }
            UnaryOperation::Atan => {
                unary_arithmetic_body!(self, mem, events, atan, [i64, u32])
            }
            UnaryOperation::Ceil => {
                unary_arithmetic_body!(self, mem, events, ceil, [i64, u32])
            }
            UnaryOperation::Cos => {
                unary_arithmetic_body!(self, mem, events, cos, [i64, u32])
            }
            UnaryOperation::Floor => {
                unary_arithmetic_body!(self, mem, events, floor, [i64, u32])
            }
            UnaryOperation::Ln => {
                unary_arithmetic_body!(self, mem, events, ln, [i64, u32])
            }
            UnaryOperation::Log10 => {
                unary_arithmetic_body!(self, mem, events, log10, [i64, u32])
            }
            UnaryOperation::Log2 => {
                unary_arithmetic_body!(self, mem, events, log2, [i64, u32])
            }
            UnaryOperation::Sin => {
                unary_arithmetic_body!(self, mem, events, sin, [i64, u32])
            }
            UnaryOperation::Sqrt => {
                unary_arithmetic_body!(self, mem, events, sqrt, [i64, u32])
            }
            UnaryOperation::Tan => {
                unary_arithmetic_body!(self, mem, events, tan, [i64, u32])
            }
            UnaryOperation::ToDegrees => {
                unary_arithmetic_body!(self, mem, events, to_degrees, [i64, u32])
            }
            UnaryOperation::ToRadians => {
                unary_arithmetic_body!(self, mem, events, to_radians, [i64, u32])
            }
        }
    }
}
impl BinaryArithmetic {
    /// Calculate the the arithmetic equation.
    /// May read values from the given memory.
    pub fn calculate(self, mem: &mut Memory, events: &mut EventWriter) -> Result<Primitive, ExecutionError> {
        use std::ops::{Add, Div, Mul, Rem, Sub};
        match self.operation {
            BinaryOperation::Add => {
                binary_arithmetic_body!(self, mem, add, events)
            }
            BinaryOperation::Mul => {
                binary_arithmetic_body!(self, mem, mul, events)
            }
            BinaryOperation::Sub => {
                binary_arithmetic_body!(self, mem, sub, events)
            }
            BinaryOperation::Div => {
                binary_arithmetic_body!(self, mem, div, events)
            }
            BinaryOperation::Mod => {
                binary_arithmetic_body!(self, mem, rem, events)
            }
            BinaryOperation::Pow => {
                binary_arithmetic_body!(self, mem, power, events)
            }
        }
    }
}
