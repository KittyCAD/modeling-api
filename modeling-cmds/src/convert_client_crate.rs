use kittycad::types as kt;

impl From<crate::ImportFile> for kt::ImportFile {
    fn from(crate::ImportFile { path, data }: crate::ImportFile) -> Self {
        Self { path, data }
    }
}

impl From<kt::ImportFile> for crate::ImportFile {
    fn from(kt::ImportFile { path, data }: kt::ImportFile) -> Self {
        Self { path, data }
    }
}

#[cfg(feature = "websocket")]
impl From<crate::websocket::ModelingSessionData> for kt::ModelingSessionData {
    fn from(crate::websocket::ModelingSessionData { api_call_id }: crate::websocket::ModelingSessionData) -> Self {
        Self { api_call_id }
    }
}

impl From<crate::units::UnitDensity> for kt::UnitDensity {
    fn from(value: crate::units::UnitDensity) -> Self {
        match value {
            crate::units::UnitDensity::PoundsPerCubicFeet => Self::LbFt3,
            crate::units::UnitDensity::KilogramsPerCubicMeter => Self::KgM3,
        }
    }
}

impl From<kt::UnitDensity> for crate::units::UnitDensity {
    fn from(value: kt::UnitDensity) -> Self {
        match value {
            kt::UnitDensity::LbFt3 => Self::PoundsPerCubicFeet,
            kt::UnitDensity::KgM3 => Self::KilogramsPerCubicMeter,
        }
    }
}
impl From<kt::UnitMass> for crate::units::UnitMass {
    fn from(value: kt::UnitMass) -> Self {
        match value {
            kt::UnitMass::G => Self::Grams,
            kt::UnitMass::Kg => Self::Kilograms,
            kt::UnitMass::Lb => Self::Pounds,
        }
    }
}
impl From<kt::UnitArea> for crate::units::UnitArea {
    fn from(value: kt::UnitArea) -> Self {
        match value {
            kt::UnitArea::Cm2 => Self::SquareCentimeters,
            kt::UnitArea::Dm2 => Self::SquareDecimeters,
            kt::UnitArea::Ft2 => Self::SquareFeet,
            kt::UnitArea::In2 => Self::SquareInches,
            kt::UnitArea::Km2 => Self::SquareKilometers,
            kt::UnitArea::M2 => Self::SquareMeters,
            kt::UnitArea::Mm2 => Self::SquareMillimeters,
            kt::UnitArea::Yd2 => Self::SquareYards,
        }
    }
}

impl From<crate::units::UnitVolume> for kt::UnitVolume {
    fn from(value: crate::units::UnitVolume) -> Self {
        match value {
            crate::units::UnitVolume::CubicCentimeters => kt::UnitVolume::Cm3,
            crate::units::UnitVolume::CubicFeet => kt::UnitVolume::Ft3,
            crate::units::UnitVolume::CubicInches => kt::UnitVolume::In3,
            crate::units::UnitVolume::CubicMeters => kt::UnitVolume::M3,
            crate::units::UnitVolume::CubicYards => kt::UnitVolume::Yd3,
            crate::units::UnitVolume::FluidOunces => kt::UnitVolume::Usfloz,
            crate::units::UnitVolume::Gallons => kt::UnitVolume::Usgal,
            crate::units::UnitVolume::Liters => kt::UnitVolume::L,
            crate::units::UnitVolume::Milliliters => kt::UnitVolume::Ml,
        }
    }
}

impl From<kt::UnitVolume> for crate::units::UnitVolume {
    fn from(value: kt::UnitVolume) -> Self {
        match value {
            kt::UnitVolume::Cm3 => crate::units::UnitVolume::CubicCentimeters,
            kt::UnitVolume::Ft3 => crate::units::UnitVolume::CubicFeet,
            kt::UnitVolume::In3 => crate::units::UnitVolume::CubicInches,
            kt::UnitVolume::M3 => crate::units::UnitVolume::CubicMeters,
            kt::UnitVolume::Yd3 => crate::units::UnitVolume::CubicYards,
            kt::UnitVolume::Usfloz => crate::units::UnitVolume::FluidOunces,
            kt::UnitVolume::Usgal => crate::units::UnitVolume::Gallons,
            kt::UnitVolume::L => crate::units::UnitVolume::Liters,
            kt::UnitVolume::Ml => crate::units::UnitVolume::Milliliters,
        }
    }
}

mod format {
    use kittycad::types as kt;

    use crate::{
        format::*,
        shared::{FileExportFormat, FileImportFormat},
    };

    impl From<FileExportFormat> for kt::FileExportFormat {
        fn from(format: FileExportFormat) -> kt::FileExportFormat {
            match format {
                FileExportFormat::Fbx => kt::FileExportFormat::Fbx,
                FileExportFormat::Glb => kt::FileExportFormat::Glb,
                FileExportFormat::Gltf => kt::FileExportFormat::Gltf,
                FileExportFormat::Obj => kt::FileExportFormat::Obj,
                FileExportFormat::Ply => kt::FileExportFormat::Ply,
                FileExportFormat::Step => kt::FileExportFormat::Step,
                FileExportFormat::Stl => kt::FileExportFormat::Stl,
            }
        }
    }

    impl From<FileImportFormat> for kt::FileImportFormat {
        fn from(format: FileImportFormat) -> kt::FileImportFormat {
            match format {
                FileImportFormat::Fbx => kt::FileImportFormat::Fbx,
                FileImportFormat::Gltf => kt::FileImportFormat::Gltf,
                FileImportFormat::Obj => kt::FileImportFormat::Obj,
                FileImportFormat::Ply => kt::FileImportFormat::Ply,
                FileImportFormat::Step => kt::FileImportFormat::Step,
                FileImportFormat::Stl => kt::FileImportFormat::Stl,
                FileImportFormat::Sldprt => kt::FileImportFormat::Sldprt,
            }
        }
    }

    impl From<InputFormat3d> for kt::InputFormat3D {
        fn from(format: InputFormat3d) -> kt::InputFormat3D {
            match format {
                InputFormat3d::Fbx(fbx::import::Options {}) => kt::InputFormat3D::Fbx {},
                InputFormat3d::Gltf(gltf::import::Options {}) => kt::InputFormat3D::Gltf {},
                InputFormat3d::Obj(obj::import::Options { coords, units }) => kt::InputFormat3D::Obj {
                    coords: coords.into(),
                    units: units.into(),
                },
                InputFormat3d::Ply(ply::import::Options { coords, units }) => kt::InputFormat3D::Ply {
                    coords: coords.into(),
                    units: units.into(),
                },
                InputFormat3d::Sldprt(sldprt::import::Options { split_closed_faces }) => {
                    kt::InputFormat3D::Sldprt { split_closed_faces }
                }
                InputFormat3d::Step(step::import::Options { split_closed_faces, .. }) => {
                    kt::InputFormat3D::Step { split_closed_faces }
                }
                InputFormat3d::Stl(stl::import::Options { coords, units }) => kt::InputFormat3D::Stl {
                    coords: coords.into(),
                    units: units.into(),
                },
            }
        }
    }

    impl From<kt::InputFormat3D> for InputFormat3d {
        fn from(value: kt::InputFormat3D) -> Self {
            match value {
                kt::InputFormat3D::Fbx {} => Self::Fbx(Default::default()),
                kt::InputFormat3D::Gltf {} => Self::Gltf(Default::default()),
                kt::InputFormat3D::Obj { coords, units } => Self::Obj(crate::format::obj::import::Options {
                    coords: coords.into(),
                    units: units.into(),
                }),
                kt::InputFormat3D::Ply { coords, units } => Self::Ply(crate::format::ply::import::Options {
                    coords: coords.into(),
                    units: units.into(),
                }),
                kt::InputFormat3D::Sldprt { split_closed_faces } => {
                    Self::Sldprt(crate::format::sldprt::import::Options { split_closed_faces })
                }
                kt::InputFormat3D::Step { split_closed_faces } => Self::Step(crate::format::step::import::Options {
                    split_closed_faces,
                    ..Default::default()
                }),
                kt::InputFormat3D::Stl { coords, units } => Self::Stl(crate::format::stl::import::Options {
                    coords: coords.into(),
                    units: units.into(),
                }),
            }
        }
    }

    impl From<crate::units::UnitLength> for kt::UnitLength {
        fn from(input: crate::units::UnitLength) -> Self {
            match input {
                crate::units::UnitLength::Centimeters => kt::UnitLength::Cm,
                crate::units::UnitLength::Feet => kt::UnitLength::Ft,
                crate::units::UnitLength::Inches => kt::UnitLength::In,
                crate::units::UnitLength::Meters => kt::UnitLength::M,
                crate::units::UnitLength::Millimeters => kt::UnitLength::Mm,
                crate::units::UnitLength::Yards => kt::UnitLength::Yd,
            }
        }
    }
    impl From<kt::UnitLength> for crate::units::UnitLength {
        fn from(input: kt::UnitLength) -> Self {
            match input {
                kt::UnitLength::Cm => Self::Centimeters,
                kt::UnitLength::Ft => Self::Feet,
                kt::UnitLength::In => Self::Inches,
                kt::UnitLength::M => Self::Meters,
                kt::UnitLength::Mm => Self::Millimeters,
                kt::UnitLength::Yd => Self::Yards,
            }
        }
    }

    impl From<crate::coord::AxisDirectionPair> for kt::AxisDirectionPair {
        fn from(input: crate::coord::AxisDirectionPair) -> kt::AxisDirectionPair {
            let axis = match input.axis {
                crate::coord::Axis::Y => kt::Axis::Y,
                crate::coord::Axis::Z => kt::Axis::Z,
            };
            let direction = match input.direction {
                crate::coord::Direction::Positive => kt::Direction::Positive,
                crate::coord::Direction::Negative => kt::Direction::Negative,
            };
            kt::AxisDirectionPair { axis, direction }
        }
    }
    impl From<kt::AxisDirectionPair> for crate::coord::AxisDirectionPair {
        fn from(input: kt::AxisDirectionPair) -> Self {
            let axis = match input.axis {
                kt::Axis::Y => crate::coord::Axis::Y,
                kt::Axis::Z => crate::coord::Axis::Z,
            };
            let direction = match input.direction {
                kt::Direction::Positive => crate::coord::Direction::Positive,
                kt::Direction::Negative => crate::coord::Direction::Negative,
            };
            Self { axis, direction }
        }
    }

    impl From<crate::coord::System> for kt::System {
        fn from(crate::coord::System { forward, up }: crate::coord::System) -> kt::System {
            kt::System {
                forward: forward.into(),
                up: up.into(),
            }
        }
    }

    impl From<kt::System> for crate::coord::System {
        fn from(kt::System { forward, up }: kt::System) -> Self {
            Self {
                forward: forward.into(),
                up: up.into(),
            }
        }
    }

    impl From<crate::ImageFormat> for kt::ImageFormat {
        fn from(format: crate::ImageFormat) -> kt::ImageFormat {
            match format {
                crate::ImageFormat::Png => kt::ImageFormat::Png,
                crate::ImageFormat::Jpeg => kt::ImageFormat::Jpeg,
            }
        }
    }
}
