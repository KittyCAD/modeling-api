//! Instruction for importing a file.
//! This will do all the file related operations, and return a
//! kittycad_modeling_cmds::ImportFiles to be passed to Endpoint::ImportFiles.

use crate::Result;
use crate::{memory::Memory, Destination, ExecutionError};
use kittycad_execution_plan_traits::{InMemory, Primitive, ReadMemory, Value};
use kittycad_modeling_cmds::{coord, format, shared, units};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::str::FromStr;

/// Data required to import a file
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ImportFiles {
    /// Which address should the imported files names be stored in, if any?
    /// Written after `format_destination`.
    pub files_destination: Option<Destination>,
    /// Which address should the imported files format be stored in, if any?
    /// Written before `files_destination`.
    pub format_destination: Option<Destination>,
    /// Look up each parameter at this address.
    /// 1: file path
    /// 2: options (file format)
    pub arguments: Vec<InMemory>,
}

impl ImportFiles {
    /// Import a file!
    pub async fn execute(self, mem: &mut Memory) -> Result<()> {
        let Self {
            files_destination,
            format_destination,
            arguments,
        } = self;

        let file_path_prim = match arguments[0] {
            InMemory::Address(addr) => mem.get_ok(&addr),
            InMemory::StackPop => mem.stack_pop(),
            InMemory::StackPeek => mem.stack_peek(),
        };

        let Ok([Primitive::String(file_path_str)]) = file_path_prim.as_deref() else {
            return Err(ExecutionError::BadArg {
                reason: "first arg must be a string".to_string(),
            });
        };

        let file_path = Path::new(&file_path_str);
        let Ok(file_contents) = fs::read(file_path) else {
            return Err(ExecutionError::General {
                reason: "Can't read file".to_string(),
            });
        };

        let ext_format = get_import_format_from_extension(file_path_str.split('.').last().ok_or_else(|| {
            ExecutionError::General {
                reason: format!("No file extension found for `{}`", file_path_str),
            }
        })?)
        .map_err(|e| ExecutionError::General { reason: e.to_string() })?;

        let options_prim = match arguments.get(1) {
            Some(InMemory::Address(addr)) => mem.get_ok(addr),
            Some(InMemory::StackPop) => mem.stack_pop(),
            Some(InMemory::StackPeek) => mem.stack_peek(),
            None => Ok(vec![]),
        };

        let maybe_opt_format = match options_prim.as_deref() {
            Ok(format_values) => from_vec_prim_to_res_opt_input_format(format_values.to_vec()),
            _ => {
                return Err(ExecutionError::General {
                    reason: "invalid format option passed".to_string(),
                });
            }
        };

        // If the "format option" was passed, check if it matches that of the file.
        let format = if let Ok(Some(opt_format)) = maybe_opt_format {
            // Validate the given format with the extension format.
            validate_extension_format(ext_format, opt_format.clone())?;
            opt_format
        } else {
            ext_format
        };

        // Get the base name (no path)
        let file_name = file_path
            .file_name()
            .map(|p| p.to_string_lossy().to_string())
            .ok_or_else(|| ExecutionError::General {
                reason: "couldn't extract file name from file path".to_string(),
            })?;

        // We're going to return possibly many files. This is because some
        // file formats have multiple side-car files.
        let mut import_files = vec![kittycad_modeling_cmds::ImportFile {
            path: file_name,
            data: file_contents.clone(),
        }];

        // In the case of a gltf importing a bin file we need to handle that! and figure out where the
        // file is relative to our current file.
        if let format::InputFormat::Gltf(format::gltf::import::Options {}) = format {
            // Check if the file is a binary gltf file, in that case we don't need to import the bin
            // file.
            if !file_contents.starts_with(b"glTF") {
                let json = gltf_json::Root::from_slice(&file_contents)
                    .map_err(|e| ExecutionError::General { reason: e.to_string() })?;

                // Read the gltf file and check if there is a bin file.
                for buffer in json.buffers {
                    if let Some(uri) = &buffer.uri {
                        if !uri.starts_with("data:") {
                            // We want this path relative to the file_path given.
                            let bin_path = std::path::Path::new(&file_path)
                                .parent()
                                .map(|p| p.join(uri))
                                .map(|p| p.to_string_lossy().to_string())
                                .ok_or_else(|| ExecutionError::General {
                                    reason: format!("Could not get the parent path of the file `{}`", file_path_str),
                                })?;

                            let bin_contents =
                                fs::read(&bin_path).map_err(|e| ExecutionError::General { reason: e.to_string() })?;

                            import_files.push(kittycad_modeling_cmds::ImportFile {
                                path: uri.to_string(),
                                data: bin_contents,
                            });
                        }
                    }
                }
            }
        }

        // Write out to memory.
        if let Some(memory_area) = format_destination {
            match memory_area {
                Destination::Address(addr) => {
                    mem.set_composite(addr, format);
                }
                Destination::StackPush => {
                    mem.stack.push(format.into_parts());
                }
                Destination::StackExtend => {
                    mem.stack.extend(format.into_parts())?;
                }
            }
        }
        if let Some(memory_area) = files_destination {
            match memory_area {
                Destination::Address(addr) => {
                    mem.set_composite(addr, import_files);
                }
                Destination::StackPush => {
                    mem.stack.push(import_files.into_parts());
                }
                Destination::StackExtend => {
                    mem.stack.extend(import_files.into_parts())?;
                }
            }
        }

        Ok(())
    }
}

/// Zoo co-ordinate system.
///
/// * Forward: -Y
/// * Up: +Z
/// * Handedness: Right
pub const ZOO_COORD_SYSTEM: coord::System = coord::System {
    forward: coord::AxisDirectionPair {
        axis: coord::Axis::Y,
        direction: coord::Direction::Negative,
    },
    up: coord::AxisDirectionPair {
        axis: coord::Axis::Z,
        direction: coord::Direction::Positive,
    },
};

fn from_vec_prim_to_res_opt_input_format(values: Vec<Primitive>) -> Result<Option<format::InputFormat>> {
    let mut iter = values.iter();

    let str_type = match iter.next() {
        None => {
            return Ok(None);
        }
        Some(Primitive::Nil) => {
            return Ok(None);
        }
        Some(Primitive::String(str)) => str,
        _ => {
            return Err(ExecutionError::General {
                reason: "missing type".to_string(),
            });
        }
    };

    return match str_type.as_str() {
        "stl" => {
            let Some(Primitive::String(str_units)) = iter.next() else {
                return Err(ExecutionError::General {
                    reason: "missing units".to_string(),
                });
            };
            let Some(Primitive::String(str_coords_forward_axis)) = iter.next() else {
                return Err(ExecutionError::General {
                    reason: "missing coords.forward.axis".to_string(),
                });
            };
            let Some(Primitive::String(str_coords_forward_direction)) = iter.next() else {
                return Err(ExecutionError::General {
                    reason: "missing coords.forward.direction".to_string(),
                });
            };
            let Some(Primitive::String(str_coords_up_axis)) = iter.next() else {
                return Err(ExecutionError::General {
                    reason: "missing coords.up.axis".to_string(),
                });
            };
            let Some(Primitive::String(str_coords_up_direction)) = iter.next() else {
                return Err(ExecutionError::General {
                    reason: "missing coords.up.direction".to_string(),
                });
            };
            Ok(Some(format::InputFormat::Stl(format::stl::import::Options {
                coords: coord::System {
                    forward: coord::AxisDirectionPair {
                        axis: coord::Axis::from_str(str_coords_forward_axis).unwrap(),
                        direction: coord::Direction::from_str(str_coords_forward_direction).unwrap(),
                    },
                    up: coord::AxisDirectionPair {
                        axis: coord::Axis::from_str(str_coords_up_axis).unwrap(),
                        direction: coord::Direction::from_str(str_coords_up_direction).unwrap(),
                    },
                },
                units: units::UnitLength::from_str(str_units).unwrap(),
            })))
        }
        _ => Err(ExecutionError::General {
            reason: "unknown type".to_string(),
        }),
    };
}

// Implemented here so we don't have to mess with kittycad::types...
fn from_input_format(type_: format::InputFormat) -> String {
    match type_ {
        format::InputFormat::Fbx(_) => "fbx".to_string(),
        format::InputFormat::Gltf(_) => "gltf".to_string(),
        format::InputFormat::Obj(_) => "obj".to_string(),
        format::InputFormat::Ply(_) => "ply".to_string(),
        format::InputFormat::Sldprt(_) => "sldprt".to_string(),
        format::InputFormat::Step(_) => "step".to_string(),
        format::InputFormat::Stl(_) => "stl".to_string(),
    }
}

fn validate_extension_format(ext: format::InputFormat, given: format::InputFormat) -> Result<()> {
    if let format::InputFormat::Stl(format::stl::import::Options { coords: _, units: _ }) = ext {
        if let format::InputFormat::Stl(format::stl::import::Options { coords: _, units: _ }) = given {
            return Ok(());
        }
    }

    if let format::InputFormat::Obj(format::obj::import::Options { coords: _, units: _ }) = ext {
        if let format::InputFormat::Obj(format::obj::import::Options { coords: _, units: _ }) = given {
            return Ok(());
        }
    }

    if let format::InputFormat::Ply(format::ply::import::Options { coords: _, units: _ }) = ext {
        if let format::InputFormat::Ply(format::ply::import::Options { coords: _, units: _ }) = given {
            return Ok(());
        }
    }

    if ext == given {
        return Ok(());
    }

    Err(ExecutionError::General {
        reason: format!(
            "The given format does not match the file extension. Expected: `{}`, Given: `{}`",
            from_input_format(ext),
            from_input_format(given)
        ),
    })
}

/// Get the source format from the extension.
fn get_import_format_from_extension(ext: &str) -> Result<format::InputFormat> {
    let format = match shared::FileImportFormat::from_str(ext) {
        Ok(format) => format,
        Err(_) => {
            if ext == "stp" {
                shared::FileImportFormat::Step
            } else if ext == "glb" {
                shared::FileImportFormat::Gltf
            } else {
                return Err(ExecutionError::General {
                    reason: format!(
                      "unknown source format for file extension: {}. Try setting the `--src-format` flag explicitly or use a valid format.",
                      ext
                    )
                });
            }
        }
    };

    // Make the default units millimeters.
    let ul = units::UnitLength::Millimeters;

    // Zoo co-ordinate system.
    //
    // * Forward: -Y
    // * Up: +Z
    // * Handedness: Right
    match format {
        shared::FileImportFormat::Step => Ok(format::InputFormat::Step(format::step::ImportOptions {})),
        shared::FileImportFormat::Stl => Ok(format::InputFormat::Stl(format::stl::import::Options {
            coords: ZOO_COORD_SYSTEM,
            units: ul,
        })),
        shared::FileImportFormat::Obj => Ok(format::InputFormat::Obj(format::obj::import::Options {
            coords: ZOO_COORD_SYSTEM,
            units: ul,
        })),
        shared::FileImportFormat::Gltf => Ok(format::InputFormat::Gltf(format::gltf::import::Options::default())),
        shared::FileImportFormat::Ply => Ok(format::InputFormat::Ply(format::ply::import::Options {
            coords: ZOO_COORD_SYSTEM,
            units: ul,
        })),
        shared::FileImportFormat::Fbx => Ok(format::InputFormat::Fbx(format::fbx::import::Options::default())),
        shared::FileImportFormat::Sldprt => Ok(format::InputFormat::Sldprt(format::sldprt::import::Options::default())),
    }
}
