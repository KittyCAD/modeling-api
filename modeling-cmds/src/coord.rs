use parse_display::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Co-ordinate axis specifier.
///
/// See [cglearn.eu] for background reading.
///
/// [cglearn.eu]: https://cglearn.eu/pub/computer-graphics/introduction-to-geometry#material-coordinate-systems-1
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr)]
#[serde(rename_all = "snake_case")]
#[display(style = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(feature = "python", pyo3::pyclass, pyo3_stub_gen::derive::gen_stub_pyclass_enum)]
pub enum Axis {
    /// 'Y' axis.
    Y = 1,
    /// 'Z' axis.
    Z = 2,
}

/// Specifies the sign of a co-ordinate axis.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr)]
#[serde(rename_all = "snake_case")]
#[display(style = "snake_case")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(feature = "python", pyo3::pyclass, pyo3_stub_gen::derive::gen_stub_pyclass_enum)]
pub enum Direction {
    /// Increasing numbers.
    Positive = 1,
    /// Decreasing numbers.
    Negative = -1,
}

impl std::ops::Mul for Direction {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match self as i32 * rhs as i32 {
            1 => Direction::Positive,
            -1 => Direction::Negative,
            _ => unreachable!(),
        }
    }
}

/// An [`Axis`] paired with a [`Direction`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr)]
#[display("({axis}, {direction})")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(feature = "python", pyo3::pyclass, pyo3_stub_gen::derive::gen_stub_pyclass)]
pub struct AxisDirectionPair {
    /// Axis specifier.
    pub axis: Axis,

    /// Specifies which direction the axis is pointing.
    pub direction: Direction,
}

/// Co-ordinate system definition.
///
/// The `up` axis must be orthogonal to the `forward` axis.
///
/// See [cglearn.eu] for background reading.
///
/// [cglearn.eu](https://cglearn.eu/pub/computer-graphics/introduction-to-geometry#material-coordinate-systems-1)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, Display, FromStr)]
#[display("forward: {forward}, up: {up}")]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export_to = "ModelingCmd.ts"))]
#[cfg_attr(feature = "python", pyo3::pyclass, pyo3_stub_gen::derive::gen_stub_pyclass)]
pub struct System {
    /// Axis the front face of a model looks along.
    pub forward: AxisDirectionPair,
    /// Axis pointing up and away from a model.
    pub up: AxisDirectionPair,
}

/// KittyCAD co-ordinate system.
///
/// * Forward: -Y
/// * Up: +Z
/// * Handedness: Right
pub const KITTYCAD: &System = &System {
    // -Y
    forward: AxisDirectionPair {
        axis: Axis::Y,
        direction: Direction::Negative,
    },
    // +Z
    up: AxisDirectionPair {
        axis: Axis::Z,
        direction: Direction::Positive,
    },
};

/// OpenGL co-ordinate system.
///
/// * Forward: +Z
/// * Up: +Y
/// * Handedness: Right
pub const OPENGL: &System = &System {
    // +Z
    forward: AxisDirectionPair {
        axis: Axis::Z,
        direction: Direction::Positive,
    },
    // +Y
    up: AxisDirectionPair {
        axis: Axis::Y,
        direction: Direction::Positive,
    },
};

/// Vulkan co-ordinate system.
///
/// * Forward: +Z
/// * Up: -Y
/// * Handedness: Left
pub const VULKAN: &System = &System {
    // +Z
    forward: AxisDirectionPair {
        axis: Axis::Z,
        direction: Direction::Positive,
    },
    // -Y
    up: AxisDirectionPair {
        axis: Axis::Y,
        direction: Direction::Negative,
    },
};

/// Perform co-ordinate system transform.
///
/// # Examples
///
/// KittyCAD (+Z up, -Y forward) to OpenGL (+Y up, +Z forward):
///
/// ```
/// # use kittycad_modeling_cmds::coord::*;
/// let a = [1.0, 2.0, 3.0];
/// let b = transform(a, KITTYCAD, OPENGL);
/// assert_eq!(b, [1.0, 3.0, -2.0]);
/// ```
///
/// OpenGL (+Y up, +Z forward) to KittyCAD (+Z up, -Y forward):
///
/// ```
/// # use kittycad_modeling_cmds::coord::*;
/// let a = [1.0, 2.0, 3.0];
/// let b = transform(a, OPENGL, KITTYCAD);
/// assert_eq!(b, [1.0, -3.0, 2.0]);
/// ```
///
/// KittyCAD (+Z up, -Y forward) to Vulkan (-Y up, +Z forward):
///
/// ```
/// # use kittycad_modeling_cmds::coord::*;
/// let a = [1.0, 2.0, 3.0];
/// let b = transform(a, KITTYCAD, VULKAN);
/// assert_eq!(b, [1.0, -3.0, -2.0]);
/// ```
///
/// OpenGL (+Y up, +Z forward) to Vulkan (-Y up, +Z forward):
///
/// ```
/// # use kittycad_modeling_cmds::coord::*;
/// let a = [1.0, 2.0, 3.0];
/// let b = transform(a, OPENGL, VULKAN);
/// assert_eq!(b, [1.0, -2.0, 3.0]);
/// ```
#[inline]
pub fn transform(a: [f32; 3], from: &System, to: &System) -> [f32; 3] {
    let mut b = a;
    b[to.forward.axis as usize] =
        (from.forward.direction * to.forward.direction) as i32 as f32 * a[from.forward.axis as usize];
    b[to.up.axis as usize] = (from.up.direction * to.up.direction) as i32 as f32 * a[from.up.axis as usize];
    b
}
