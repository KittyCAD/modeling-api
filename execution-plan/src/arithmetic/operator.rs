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
    /// Modulo
    Mod,
    /// Power
    Pow,
    /// Logarithm
    Log,
    /// Smallest of two numbers
    Min,
    /// Largest of two numbers
    Max,
}

/// Operations that can be applied to a value in memory, requiring one operand.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperation {
    /// Logical negation
    Not,
    /// Flip the sign of a signed number
    Neg,
    /// Get the absolute value of a number
    Abs,
    /// Arc cosine
    Acos,
    /// Arc sine
    Asin,
    /// Arc tangent
    Atan,
    /// Ceiling
    Ceil,
    /// Cosine
    Cos,
    /// Floor,
    Floor,
    /// Natural logarithm
    Ln,
    /// Logarithm base 10
    Log10,
    /// Logarithm base 2
    Log2,
    /// Sine
    Sin,
    /// Square root
    Sqrt,
    /// Tangent
    Tan,
    /// Convert radians to degrees
    ToDegrees,
    /// Convert degrees to radians
    ToRadians,
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
            UnaryOperation::Abs => "abs",
            UnaryOperation::Acos => "acos",
            UnaryOperation::Asin => "asin",
            UnaryOperation::Atan => "atan",
            UnaryOperation::Ceil => "ceil",
            UnaryOperation::Cos => "cos",
            UnaryOperation::Floor => "floor",
            UnaryOperation::Ln => "ln",
            UnaryOperation::Log10 => "log10",
            UnaryOperation::Log2 => "log2",
            UnaryOperation::Sin => "sin",
            UnaryOperation::Sqrt => "sqrt",
            UnaryOperation::Tan => "tan",
            UnaryOperation::ToDegrees => "to_degrees",
            UnaryOperation::ToRadians => "to_radians",
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
            BinaryOperation::Mod => "%",
            BinaryOperation::Pow => "^",
            BinaryOperation::Log => "log",
            BinaryOperation::Min => "min",
            BinaryOperation::Max => "max",
        }
        .fmt(f)
    }
}
