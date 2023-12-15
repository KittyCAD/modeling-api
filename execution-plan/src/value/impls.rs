//! General principles for implementing Value
//!
//! - Each struct should have a canonical ordering for its fields.
//! - Always lay these fields out in the canonical ordering.
//! - This canonical ordering is the order of the struct's fields in its Rust source code definition.
//! - Enums get laid out by first putting the variant as a string, then putting the variant's fields.
use kittycad_modeling_cmds::{
    ok_response::OkModelingCmdResponse,
    output,
    shared::{PathSegment, Point2d, Point3d},
};

use super::Value;
use crate::{Address, ExecutionError, Primitive};

const EMPTY: &str = "EMPTY";
const TAKE_SNAPSHOT: &str = "TAKE_SNAPSHOT";
const ARC: &str = "arc";
const LINE: &str = "line";
const TAN_ARC: &str = "tan_arc";
const TAN_ARC_TO: &str = "tan_arc_to";
const BEZIER: &str = "bezier";

impl<T> Value for Point2d<T>
where
    Primitive: From<T>,
    T: TryFrom<Primitive, Error = ExecutionError>,
{
    fn into_parts(self) -> Vec<Primitive> {
        let points = [self.x, self.y];
        points.into_iter().map(|component| component.into()).collect()
    }

    fn from_parts(values: &[Option<Primitive>]) -> Result<Self, ExecutionError> {
        let err = || ExecutionError::MemoryWrongSize { expected: 2 };
        let [x, y] = [0, 1].map(|n| values.get(n).ok_or(err()));
        let x = x?
            .to_owned()
            .ok_or(ExecutionError::MemoryWrongSize { expected: 2 })?
            .try_into()?;
        let y = y?
            .to_owned()
            .ok_or(ExecutionError::MemoryWrongSize { expected: 2 })?
            .try_into()?;
        Ok(Self { x, y })
    }
}

impl<T> Value for Point3d<T>
where
    Primitive: From<T>,
    T: TryFrom<Primitive, Error = ExecutionError>,
{
    fn into_parts(self) -> Vec<Primitive> {
        let points = [self.x, self.y, self.z];
        points.into_iter().map(|component| component.into()).collect()
    }

    fn from_parts(values: &[Option<Primitive>]) -> Result<Self, ExecutionError> {
        let err = || ExecutionError::MemoryWrongSize { expected: 3 };
        let [x, y, z] = [0, 1, 2].map(|n| values.get(n).ok_or(err()));
        let x = x?
            .to_owned()
            .ok_or(ExecutionError::MemoryWrongSize { expected: 3 })?
            .try_into()?;
        let y = y?
            .to_owned()
            .ok_or(ExecutionError::MemoryWrongSize { expected: 3 })?
            .try_into()?;
        let z = z?
            .to_owned()
            .ok_or(ExecutionError::MemoryWrongSize { expected: 3 })?
            .try_into()?;
        Ok(Self { x, y, z })
    }
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
            _ => todo!(),
        }
    }

    fn from_parts(values: &[Option<Primitive>]) -> Result<Self, ExecutionError> {
        let variant_name: String = get_some(values, 0)?.try_into()?;
        match variant_name.as_str() {
            EMPTY => Ok(OkModelingCmdResponse::Empty),
            TAKE_SNAPSHOT => {
                let contents: Vec<u8> = get_some(values, 1)?.try_into()?;
                Ok(OkModelingCmdResponse::TakeSnapshot(output::TakeSnapshot {
                    contents: contents.into(),
                }))
            }
            _ => todo!(),
        }
    }
}

/// Layout: A single memory address, storing the snapshot's bytes as a primitive.
impl Value for output::TakeSnapshot {
    fn into_parts(self) -> Vec<Primitive> {
        vec![Primitive::Bytes(self.contents.into())]
    }

    fn from_parts(values: &[Option<Primitive>]) -> Result<Self, ExecutionError> {
        let contents: Vec<u8> = get_some(values, 0)?.try_into()?;
        Ok(Self {
            contents: contents.into(),
        })
    }
}

fn get_some(values: &[Option<Primitive>], i: usize) -> Result<Primitive, ExecutionError> {
    let addr = Address(0); // TODO: pass the `start` addr in
    let v = values.get(i).ok_or(ExecutionError::MemoryEmpty { addr })?.to_owned();
    let v = v.ok_or(ExecutionError::MemoryEmpty { addr })?.to_owned();
    Ok(v)
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

    fn from_parts(values: &[Option<Primitive>]) -> Result<Self, ExecutionError> {
        let variant_name: String = get_some(values, 0)?.try_into()?;
        match variant_name.as_str() {
            LINE => {
                let end = Point3d::from_parts(values)?;
                let relative = get_some(values, 1)?.try_into()?;
                Ok(Self::Line { end, relative })
            }
            ARC => {
                todo!()
            }
            BEZIER => {
                todo!()
            }
            TAN_ARC => {
                todo!()
            }
            TAN_ARC_TO => {
                todo!()
            }
            other => Err(ExecutionError::InvalidEnumVariant {
                expected_type: "line segment".to_owned(),
                actual: other.to_owned(),
            }),
        }
    }
}
