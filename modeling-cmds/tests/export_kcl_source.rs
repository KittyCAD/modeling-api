//! Compatibility checks for optional KCL source on export commands.

use std::collections::BTreeMap;

use kittycad_modeling_cmds::{
    each_cmd::{Export, Export3d},
    format::{step, OutputFormat3d},
    shared::KclSource,
    ModelingCmd,
};
use serde_json::json;

#[test]
fn existing_exports_omit_source_and_remain_readable() {
    let format = OutputFormat3d::Step(step::export::Options::default());
    for (kind, command) in [
        (
            "export",
            ModelingCmd::from(Export::builder().format(format.clone()).build()),
        ),
        (
            "export3d",
            ModelingCmd::from(Export3d::builder().format(format.clone()).build()),
        ),
    ] {
        let old_request = json!({"type": kind, "entity_ids": [], "format": format});
        assert_eq!(serde_json::to_value(&command).unwrap(), old_request);
        assert_eq!(
            serde_json::from_value::<ModelingCmd>(old_request.clone()).unwrap(),
            command
        );

        let mut explicit_null = old_request;
        explicit_null["kcl_source"] = serde_json::Value::Null;
        assert_eq!(serde_json::from_value::<ModelingCmd>(explicit_null).unwrap(), command);
    }
}

#[test]
fn exports_preserve_multifile_source() {
    let source = KclSource::builder()
        .entrypoint("main.kcl".into())
        .files(BTreeMap::from([
            (
                "main.kcl".into(),
                "import helper from 'lib/helper.kcl'\r\n// café 零 🦆 \\".into(),
            ),
            ("lib/helper.kcl".into(), "// no trailing newline".into()),
            ("empty.kcl".into(), String::new()),
        ]))
        .build();
    let format = OutputFormat3d::Step(step::export::Options::default());
    for command in [
        ModelingCmd::from(
            Export::builder()
                .format(format.clone())
                .kcl_source(source.clone())
                .build(),
        ),
        ModelingCmd::from(Export3d::builder().format(format).kcl_source(source.clone()).build()),
    ] {
        let wire = serde_json::to_value(&command).unwrap();
        assert_eq!(wire["kcl_source"], serde_json::to_value(&source).unwrap());
        assert_eq!(serde_json::from_value::<ModelingCmd>(wire).unwrap(), command);
    }
}

#[test]
fn export_schema_keeps_source_optional() {
    for schema in [schemars::schema_for!(Export), schemars::schema_for!(Export3d)] {
        let object = schema.schema.object.unwrap();
        assert!(!object.required.contains("kcl_source"));
        assert!(object.properties.contains_key("kcl_source"));
        let source = serde_json::to_value(&schema.definitions["KclSource"]).unwrap();
        assert_eq!(source["required"], json!(["entrypoint", "files"]));
        assert_eq!(source["properties"]["files"]["additionalProperties"]["type"], "string");
    }
}

#[cfg(feature = "ts-rs")]
#[test]
fn export_typescript_includes_optional_source() {
    use ts_rs::{Config, TS};

    let config = Config::default().with_out_dir("../target/export-kcl-bindings");
    assert!(Export::decl(&config).contains("kcl_source?: KclSource"));
    assert!(Export3d::decl(&config).contains("kcl_source?: KclSource"));
    assert!(KclSource::decl(&config).contains("entrypoint: string"));
    Export::export_all(&config).unwrap();
    Export3d::export_all(&config).unwrap();
}
