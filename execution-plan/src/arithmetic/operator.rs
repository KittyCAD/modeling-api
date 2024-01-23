use serde::{Deserialize, Serialize};
use std::fmt;

/// Operations that can be applied to values in memory.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    /// Requires one operand (e.g. negating a number)
    Unary(UnaryOperation),
    /// Requires two operands (e.g. addition)
    Binary(BinaryOperation),
}

/// Operations that can be applied to values in memory, requiring two operands.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperation {
    /// Addition
    Add,
    /// Multiplication
    Mul,
    /// Subtraction
    Sub,
    /// Division
    Div,
}

/// Operations that can be applied to a value in memory, requiring one operand.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperation {
    /// Logical negation
    Not,
    /// Flip the sign of a signed number
    Neg,
}

impl From<BinaryOperation> for Operation {
    fn from(value: BinaryOperation) -> Self {
        Self::Binary(value)
    }
}

impl From<UnaryOperation> for Operation {
    fn from(value: UnaryOperation) -> Self {
        Self::Unary(value)
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Unary(o) => o.fmt(f),
            Operation::Binary(o) => o.fmt(f),
        }
    }
}

impl fmt::Display for UnaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperation::Neg => "-",
            UnaryOperation::Not => "!",
        }
        .fmt(f)
    }
}

impl fmt::Display for BinaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperation::Add => "+",
            BinaryOperation::Mul => "*",
            BinaryOperation::Sub => "-",
            BinaryOperation::Div => "/",
        }
        .fmt(f)
    }
}
