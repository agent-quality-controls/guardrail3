use super::helpers::{arch_01_errors, copy_golden, run_check, write_file};

#[test]
fn crate_not_in_workspace_members() {
    let tmp = copy_golden();
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/events/Cargo.toml",
        "[package]\nname = \"devctl-domain-events\"\nversion = \"0.1.0\"\nedition = \"2024\"",
    );
    write_file(tmp.path(), "apps/devctl/crates/domain/events/src/lib.rs", "// events");
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("not a workspace member") || e.title.contains("not in workspace")),
        "expected error about crate not being a workspace member, got: {errors:#?}"
    );
}
