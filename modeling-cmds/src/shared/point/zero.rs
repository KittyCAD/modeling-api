use super::{Point2d, Point3d, Point4d};

macro_rules! impl_zero {
    ($t:ident, $zero:literal) => {
        impl Point2d<$t> {
            /// Set all components to zero.
            pub const fn zero() -> Self {
                Self::uniform($zero)
            }
        }
        impl Point3d<$t> {
            /// Set all components to zero.
            pub const fn zero() -> Self {
                Self::uniform($zero)
            }
        }
        impl Point4d<$t> {
            /// Set all components to zero.
            pub const fn zero() -> Self {
                Self::uniform($zero)
            }
        }
    };
}

impl_zero!(u8, 0);
impl_zero!(u16, 0);
impl_zero!(u32, 0);
impl_zero!(u64, 0);
impl_zero!(u128, 0);
impl_zero!(f32, 0.0);
impl_zero!(f64, 0.0);
