use super::helpers::{arch_errors, copy_fixture, run_check, write_file};

#[test]
fn workspace_has_extra_member() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        "[workspace]\nmembers = [\n    \"crates/domain/types\",\n    \"crates/app/core\",\n    \"crates/ports/outbound/traits\",\n    \"crates/adapters/inbound/cli\",\n    \"crates/adapters/outbound/fs\",\n    \"crates/domain/phantom\",\n]\nresolver = \"2\"",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("phantom") || e.title.contains("extra member") || e.title.contains("does not exist")),
        "expected error about phantom workspace member, got: {errors:#?}"
    );
}
