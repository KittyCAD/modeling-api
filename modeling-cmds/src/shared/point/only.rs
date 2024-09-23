use super::{Point2d, Point3d, Point4d};

macro_rules! impl_only {
    ($typ:ident, $method:ident, $component:ident, $($i:ident),*) => {
        impl<T> $typ<T>
        where
            T: Default,
        {
            /// Set only the given component.
            /// All other components have their default value
            pub fn $method($component: T) -> Self {
                Self {
                    $component,
                    $(
                        $i: Default::default(),
                    )*
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        assert_eq!(Point2d::only_x(1), Point2d { x: 1, y: 0 });
        assert_eq!(Point2d::only_y(1), Point2d { x: 0, y: 1 });

        assert_eq!(Point3d::only_x(1), Point3d { x: 1, y: 0, z: 0 });
        assert_eq!(Point3d::only_y(1), Point3d { x: 0, y: 1, z: 0 });
        assert_eq!(Point3d::only_z(1), Point3d { x: 0, y: 0, z: 1 });

        assert_eq!(Point4d::only_x(1), Point4d { x: 1, y: 0, z: 0, w: 0 });
        assert_eq!(Point4d::only_y(1), Point4d { x: 0, y: 1, z: 0, w: 0 });
        assert_eq!(Point4d::only_z(1), Point4d { x: 0, y: 0, z: 1, w: 0 });
        assert_eq!(Point4d::only_w(1), Point4d { x: 0, y: 0, z: 0, w: 1 });
    }
}

impl_only!(Point2d, only_x, x, y);
impl_only!(Point2d, only_y, y, x);
impl_only!(Point3d, only_x, x, y, z);
impl_only!(Point3d, only_y, y, x, z);
impl_only!(Point3d, only_z, z, x, y);
impl_only!(Point4d, only_x, x, y, z, w);
impl_only!(Point4d, only_y, y, x, z, w);
impl_only!(Point4d, only_z, z, x, y, w);
impl_only!(Point4d, only_w, w, x, y, z);
