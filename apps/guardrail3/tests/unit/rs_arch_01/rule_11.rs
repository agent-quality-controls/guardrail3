use super::helpers::{arch_errors, copy_fixture, run_check, write_file};

#[test]
fn root_workspace_includes_app() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "Cargo.toml",
        "[workspace]\nmembers = [\"packages/shared-types\", \"apps/devctl\"]\nresolver = \"2\"",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("root workspace") || e.title.contains("apps/devctl")),
        "expected error about root workspace including app, got: {errors:#?}"
    );
}
