#![cfg(feature = "python")]
#![allow(missing_docs)]

use std::{collections::HashSet, path::PathBuf};

// Verify that PyO3 renames via `#[pyclass(name = "...")]` are visible to
// the stub generator (pyo3-stub-gen). This prevents duplicate class names like
// `Options` from leaking into generated .pyi stubs.
#[test]
fn pyclass_renames_are_used_in_stub_generation() {
    // Touch the types so the linker keeps the object files that contain
    // the `inventory::submit!` registrations emitted by the derive macros.
    use kittycad_modeling_cmds::format;
    let _ = std::any::TypeId::of::<format::dxf::export::Options>();
    let _ = std::any::TypeId::of::<format::obj::import::Options>();
    let _ = std::any::TypeId::of::<format::obj::export::Options>();
    let _ = std::any::TypeId::of::<format::stl::import::Options>();
    let _ = std::any::TypeId::of::<format::stl::export::Options>();
    // Collect stub info registered by `gen_stub_*` derives in this crate.
    // We don't require a real pyproject.toml for this check.
    let info = pyo3_stub_gen::StubInfo::from_project_root(
        "kittycad_modeling_cmds".to_string(),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    )
    .expect("stub info should be collectable with python feature enabled");

    // Gather all class names across all modules.
    let mut names = HashSet::new();
    for module in info.modules.values() {
        eprintln!("module {}", module.name);
        for class in module.class.values() {
            eprintln!("  class {}", class.name);
            names.insert(class.name);
        }
    }

    // Spot-check a few renamed classes that were previously `Options` in Rust.
    // If pyo3-stub-gen fails to honor `pyclass(name = ...)`, these would appear as
    // `Options` and collide across modules.
    for expected in [
        "DxfExportOptions",
        "ObjImportOptions",
        "ObjExportOptions",
        "StlImportOptions",
        "StlExportOptions",
    ] {
        assert!(
            names.contains(expected),
            "expected renamed class `{}` to appear in stub info; got {:?}",
            expected,
            names
        );
    }
}
