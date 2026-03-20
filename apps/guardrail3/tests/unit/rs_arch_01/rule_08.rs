use super::helpers::{arch_errors, copy_fixture, run_check, write_file};

#[test]
fn app_cargo_toml_not_workspace() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        "[package]\nname = \"devctl\"\nversion = \"0.1.0\"\nedition = \"2024\"",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("not a workspace") || e.title.contains("must be a workspace")),
        "expected error about app Cargo.toml not being a workspace, got: {errors:#?}"
    );
}
