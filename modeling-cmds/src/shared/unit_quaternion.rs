//! Quaternion used for rotations whose magnitude is guaranteed to be between 0 and 1 (inclusive)
use kittycad_point::{Point3d, Point4d};

use crate::shared::Angle;

/// A 4D point whose magnitude is between 0 and 1.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnitQuaternion<T> {
    inner: Point4d<T>,
}

/// Errors that could occur when making a unit quaternion
#[derive(Debug)]
pub enum UnitQuaternionError {
    /// One or more components were not >= 0, or <= 1
    IsNotUnit,
}

impl UnitQuaternion<f64> {
    /// Make a new unit quaternion, if every component of the point is between 0 and 1 inclusive.
    pub fn new(Point4d { x, y, z, w }: Point4d<f64>) -> Result<Self, UnitQuaternionError> {
        let mag = (x.powi(2) + y.powi(2) + z.powi(2) + w.powi(2)).sqrt();
        let is_unit = (0.0..=1.0).contains(&mag);
        if !is_unit {
            return Err(UnitQuaternionError::IsNotUnit);
        }
        Ok(Self {
            inner: Point4d { x, y, z, w },
        })
    }
}

impl UnitQuaternion<f64> {
    /// Express this as a rotation about an axis, of a certain angle.
    pub fn to_axis_angle(
        Self {
            inner: Point4d { x, y, z, w },
        }: Self,
    ) -> (Point3d<f64>, Angle) {
        let theta_radians = 2.0 * w.acos();
        let theta = Angle::from_radians(theta_radians);
        let s = (1.0 - w.powi(2)).sqrt();
        let axis = if s != 0.0 {
            Point3d {
                x: x / s,
                y: y / s,
                z: z / s,
            }
        } else {
            Point3d { x: 1.0, y: 0.0, z: 0.0 }
        };
        (axis, theta)
    }

    /// Construct from a rotation about an axis, of a certain angle.
    pub fn new_from_axis_angle(axis: Point3d<f64>, angle: Angle) -> Result<Self, UnitQuaternionError> {
        let theta = angle.to_radians();
        let half_theta = theta / 2.0;
        let w = half_theta.cos();
        let s = half_theta.sin();
        let x = axis.x * s;
        let y = axis.y * s;
        let z = axis.z * s;
        Self::new(Point4d { x, y, z, w })
    }
}
