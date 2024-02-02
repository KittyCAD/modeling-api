use serde::{Deserialize, Serialize};
use std::fmt;

/// An address in KCEP's program memory.
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Address(pub(crate) usize);

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Addr{}", self.0)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Address {
    /// First memory address available.
    pub const ZERO: Self = Self(0);

    /// Offset the memory by `size` addresses.
    pub fn offset(self, size: usize) -> Self {
        let curr = self.0;
        Self(curr + size)
    }

    /// Returns self, then offsets self by `size` addresses.
    pub fn offset_by(&mut self, size: usize) -> Self {
        let old = *self;
        self.0 += size;
        old
    }
}

/// Offset the address.
impl std::ops::Add<usize> for Address {
    type Output = Self;

    /// Offset the address.
    fn add(self, rhs: usize) -> Self::Output {
        self.offset(rhs)
    }
}

/// Offset the address.
impl std::ops::AddAssign<usize> for Address {
    /// Offset the address.
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

/// Offset the address.
impl std::ops::Add for Address {
    type Output = Self;

    /// Offset the address.
    fn add(self, rhs: Self) -> Self::Output {
        self.offset(rhs.0)
    }
}

/// Find the offset between two addresses.
impl std::ops::Sub for Address {
    type Output = usize;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl From<usize> for Address {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
