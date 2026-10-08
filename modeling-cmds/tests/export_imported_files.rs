//! Compatibility and byte-preservation tests for original files supplied with exports.

use kittycad_modeling_cmds::{
    each_cmd::{Export, Export3d},
    format::{gltf::export::Options, OutputFormat3d},
    ImportFile, ModelingCmd,
};
use serde_json::json;

#[test]
fn old_export_requests_and_builders_omit_imported_files() {
    let format = OutputFormat3d::Gltf(Options::default());
    for command in [
        ModelingCmd::from(Export3d::builder().format(format.clone()).build()),
        ModelingCmd::from(Export::builder().format(format.clone()).build()),
    ] {
        let value = serde_json::to_value(&command).unwrap();
        assert!(value.get("imported_files").is_none());
        assert_eq!(serde_json::from_value::<ModelingCmd>(value).unwrap(), command);
    }
}

#[test]
fn both_export_commands_round_trip_original_binary_files() {
    let files = vec![
        ImportFile::builder()
            .path("parts/reference.step".to_owned())
            .data(vec![0, 255, 128, 13, 10])
            .build(),
        ImportFile::builder()
            .path("parts/empty.bin".to_owned())
            .data(Vec::new())
            .build(),
    ];
    let format = OutputFormat3d::Gltf(Options::default());
    for command in [
        ModelingCmd::from(
            Export3d::builder()
                .format(format.clone())
                .imported_files(files.clone())
                .build(),
        ),
        ModelingCmd::from(
            Export::builder()
                .format(format.clone())
                .imported_files(files.clone())
                .build(),
        ),
    ] {
        let value = serde_json::to_value(&command).unwrap();
        assert_eq!(
            value["imported_files"],
            json!([
                {"path": "parts/reference.step", "data": [0, 255, 128, 13, 10]},
                {"path": "parts/empty.bin", "data": []}
            ])
        );
        assert_eq!(serde_json::from_value::<ModelingCmd>(value).unwrap(), command);
        let bson = bson::to_vec(&command).unwrap();
        assert_eq!(bson::from_slice::<ModelingCmd>(&bson).unwrap(), command);
        assert!(format!("{command:?}").contains("<redacted>"));
    }
}
