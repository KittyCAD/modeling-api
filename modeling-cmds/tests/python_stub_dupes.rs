//! Checks for duplicate class/enum names in generated stubs.
#![cfg(feature = "python")]

use std::{collections::BTreeMap, path::PathBuf};

#[test]
fn print_duplicate_stub_names() {
    // Force-link some representative types; inventory should still see all registrants.
    use kittycad_modeling_cmds as _;

    let info = pyo3_stub_gen::StubInfo::from_project_root(
        "kittycad_modeling_cmds".to_string(),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    )
    .expect("collect stub info");

    let mut class_counts: BTreeMap<&str, usize> = BTreeMap::new();
    let mut enum_counts: BTreeMap<&str, usize> = BTreeMap::new();

    for module in info.modules.values() {
        for class in module.class.values() {
            *class_counts.entry(class.name).or_default() += 1;
        }
        for en in module.enum_.values() {
            *enum_counts.entry(en.name).or_default() += 1;
        }
    }

    let dup_classes: Vec<_> = class_counts
        .iter()
        .filter(|(_, &c)| c > 1)
        .map(|(n, c)| (n.to_string(), c))
        .collect();
    let dup_enums: Vec<_> = enum_counts
        .iter()
        .filter(|(_, &c)| c > 1)
        .map(|(n, c)| (n.to_string(), c))
        .collect();

    eprintln!("duplicate classes: {:?}", dup_classes);
    eprintln!("duplicate enums: {:?}", dup_enums);

    // This is informational; do not fail test. If we decide to enforce, we can assert emptiness.
}
