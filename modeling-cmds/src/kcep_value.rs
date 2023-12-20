use kittycad_execution_plan_traits::{MemoryError, Primitive, Value};

use crate::{
    ok_response::OkModelingCmdResponse,
    output,
    shared::{Angle, PathSegment, Point2d, Point3d},
};

pub(crate) const EMPTY: &str = "EMPTY";
pub(crate) const TAKE_SNAPSHOT: &str = "TAKE_SNAPSHOT";
pub(crate) const ARC: &str = "arc";
pub(crate) const LINE: &str = "line";
pub(crate) const TAN_ARC: &str = "tan_arc";
pub(crate) const TAN_ARC_TO: &str = "tan_arc_to";
pub(crate) const BEZIER: &str = "bezier";

fn err() -> MemoryError {
    MemoryError::MemoryWrongSize
}

/// Layout:
/// - One memory address to store the variant name
/// - Following memory addresses to store the variant's single field.
impl Value for OkModelingCmdResponse {
    fn into_parts(self) -> Vec<Primitive> {
        match self {
            OkModelingCmdResponse::Empty => vec![Primitive::String(EMPTY.to_string())],
            OkModelingCmdResponse::TakeSnapshot(snap) => {
                let mut parts = vec![Primitive::String(TAKE_SNAPSHOT.to_owned())];
                parts.extend(snap.into_parts());
                parts
            }
            _ => todo!("Implement Value for more OkModelingCmdResponse variants"),
        }
    }

    fn from_parts<I>(values: &mut I) -> Result<Self, MemoryError>
    where
        I: Iterator<Item = Option<Primitive>>,
    {
        let variant_name: String = next(values)?;
        match variant_name.as_str() {
            EMPTY => Ok(OkModelingCmdResponse::Empty),
            TAKE_SNAPSHOT => {
                let contents: Vec<u8> = next(values)?;
                Ok(OkModelingCmdResponse::TakeSnapshot(output::TakeSnapshot {
                    contents: contents.into(),
                }))
            }
            _ => todo!("Implement Value for more OkModelingCmdResponse variants"),
        }
    }
}

/// Layout: A single memory address, storing the snapshot's bytes as a primitive.
impl Value for output::TakeSnapshot {
    fn into_parts(self) -> Vec<Primitive> {
        vec![Primitive::Bytes(self.contents.into())]
    }

    fn from_parts<I>(values: &mut I) -> Result<Self, MemoryError>
    where
        I: Iterator<Item = Option<Primitive>>,
    {
        let contents: Vec<u8> = next(values)?;
        Ok(Self {
            contents: contents.into(),
        })
    }
}

/// Read the next primitive.
/// If it's
fn next<I, T>(values: &mut I) -> Result<T, MemoryError>
where
    I: Iterator<Item = Option<Primitive>>,
    T: TryFrom<Primitive, Error = MemoryError>,
{
    values.next().ok_or_else(err)?.ok_or_else(err)?.try_into()
}

/// Layout:
/// - One memory address to store the variant name
/// - Following memory addresses to store the variant's fields.
impl Value for PathSegment {
    fn into_parts(self) -> Vec<Primitive> {
        let name: String = match &self {
            PathSegment::Line { .. } => LINE.to_owned(),
            PathSegment::Arc { .. } => ARC.to_owned(),
            PathSegment::Bezier { .. } => BEZIER.to_owned(),
            PathSegment::TangentialArc { .. } => TAN_ARC.to_owned(),
            PathSegment::TangentialArcTo { .. } => TAN_ARC_TO.to_owned(),
        };
        let name = Primitive::from(name);
        let data = match self {
            PathSegment::Line { end, relative } => {
                let mut parts = end.into_parts();
                parts.push(relative.into());
                parts
            }
            PathSegment::Arc {
                center,
                radius,
                start,
                end,
                relative,
            } => {
                let mut parts = center.into_parts();
                parts.push(radius.into());
                parts.push(start.into());
                parts.push(end.into());
                parts.push(relative.into());
                parts
            }
            PathSegment::Bezier {
                control1,
                control2,
                end,
                relative,
            } => {
                let mut parts = control1.into_parts();
                parts.extend(control2.into_parts());
                parts.extend(end.into_parts());
                parts.push(relative.into());
                parts
            }
            PathSegment::TangentialArc { radius, offset } => {
                vec![radius.into(), offset.into()]
            }
            PathSegment::TangentialArcTo {
                to,
                angle_snap_increment,
            } => {
                let mut parts = to.into_parts();
                parts.push(match angle_snap_increment {
                    Some(angle) => angle.into(),
                    None => Primitive::Nil,
                });
                parts
            }
        };
        let mut parts = Vec::with_capacity(1 + data.len());
        parts.push(name);
        parts.extend(data);
        parts
    }

    fn from_parts<I>(values: &mut I) -> Result<Self, MemoryError>
    where
        I: Iterator<Item = Option<Primitive>>,
    {
        let variant_name: String = next(values)?;
        match variant_name.as_str() {
            LINE => {
                let end = Point3d::from_parts(values)?;
                let relative = next(values)?;
                Ok(Self::Line { end, relative })
            }
            ARC => {
                let center = Point2d::from_parts(values)?;
                let radius = Primitive::from_parts(values)?.try_into()?;
                let start = Primitive::from_parts(values)?.try_into()?;
                let end = Primitive::from_parts(values)?.try_into()?;
                let relative = Primitive::from_parts(values)?.try_into()?;
                Ok(Self::Arc {
                    center,
                    radius,
                    start,
                    end,
                    relative,
                })
            }
            BEZIER => {
                let control1 = Point3d::from_parts(values)?;
                let control2 = Point3d::from_parts(values)?;
                let end = Point3d::from_parts(values)?;
                let relative = Primitive::from_parts(values)?.try_into()?;
                Ok(Self::Bezier {
                    control1,
                    control2,
                    end,
                    relative,
                })
            }
            TAN_ARC => {
                let radius = Primitive::from_parts(values).and_then(f64::try_from)?;
                let offset = Primitive::from_parts(values).and_then(Angle::try_from)?;
                Ok(Self::TangentialArc { radius, offset })
            }
            TAN_ARC_TO => {
                let to = Point3d::from_parts(values)?;
                let angle_snap_increment = if let Some(Some(primitive)) = values.next() {
                    Some(Angle::try_from(primitive)?)
                } else {
                    None
                };
                Ok(Self::TangentialArcTo {
                    to,
                    angle_snap_increment,
                })
            }
            other => Err(MemoryError::InvalidEnumVariant {
                expected_type: "line segment".to_owned(),
                actual: other.to_owned(),
            }),
        }
    }
}
