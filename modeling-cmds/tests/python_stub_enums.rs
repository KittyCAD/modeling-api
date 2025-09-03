//! Ensures renamed `Storage` enums are reflected in stub generation.
#![cfg(feature = "python")]

use std::{collections::HashSet, path::PathBuf};

#[test]
fn pyclass_enum_renames_are_used_in_stub_generation() {
    // Touch the enums so the linker keeps the object files that contain
    // the `inventory::submit!` registrations emitted by the derive macros.
    use kittycad_modeling_cmds::format;
    let _ = std::any::TypeId::of::<format::dxf::export::Storage>();
    let _ = std::any::TypeId::of::<format::stl::export::Storage>();
    let _ = std::any::TypeId::of::<format::ply::export::Storage>();
    let _ = std::any::TypeId::of::<format::fbx::export::Storage>();
    let _ = std::any::TypeId::of::<format::gltf::export::Storage>();

    // Build stub info from this crate's project root.
    let info = pyo3_stub_gen::StubInfo::from_project_root(
        "kittycad_modeling_cmds".to_string(),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    )
    .expect("stub info should be collectable with python feature enabled");

    // Gather all enum names across all modules.
    let mut enum_names = HashSet::new();
    for module in info.modules.values() {
        for en in module.enum_.values() {
            enum_names.insert(en.name);
        }
    }

    // These enums used to be exported as `Storage` which collided.
    for expected in ["DxfStorage", "StlStorage", "PlyStorage", "FbxStorage", "GltfStorage"] {
        assert!(
            enum_names.contains(expected),
            "expected renamed enum `{}` to appear in stub info; got {:?}",
            expected,
            enum_names
        );
    }
}
