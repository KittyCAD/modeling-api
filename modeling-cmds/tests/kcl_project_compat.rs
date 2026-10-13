//! Checks that moving KCL project types preserves consumers' existing import paths.

use kittycad_modeling_cmds::{kcl_project, shared};

#[test]
fn shared_project_types_and_builders_remain_compatible() {
    let file_builder: shared::KclFileBuilder = kcl_project::KclFile::builder();
    let file: shared::KclFile = file_builder
        .path("main.kcl".parse().unwrap())
        .contents(b"// KCL source".to_vec())
        .build();
    let file: kcl_project::KclFile = file;

    let project_builder: shared::KclProjectBuilder = kcl_project::KclProject::builder();
    let project: shared::KclProject = project_builder
        .files(vec![file])
        .entrypoint("main.kcl".parse().unwrap())
        .build();
    let project: kcl_project::KclProject = project;

    assert_eq!(project.entrypoint, project.files[0].path);
}

#[cfg(feature = "exec-kcl")]
#[test]
fn exec_kcl_project_types_and_builders_remain_compatible() {
    use kittycad_modeling_cmds::exec_kcl;

    let file_builder: exec_kcl::KclFileBuilder = kcl_project::KclFile::builder();
    let file: exec_kcl::KclFile = file_builder
        .path("main.kcl".parse().unwrap())
        .contents(b"// KCL source".to_vec())
        .build();
    let file: kcl_project::KclFile = file;

    let project_builder: exec_kcl::KclProjectBuilder = kcl_project::KclProject::builder();
    let project: exec_kcl::KclProject = project_builder
        .files(vec![file])
        .entrypoint("main.kcl".parse().unwrap())
        .build();
    let project: kcl_project::KclProject = project;

    assert_eq!(project.entrypoint, project.files[0].path);
}
