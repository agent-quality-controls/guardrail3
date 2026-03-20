use super::helpers::{arch_01_errors, copy_golden, run_check, write_file};

#[test]
fn workspace_member_outside_app() {
    let tmp = copy_golden();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        "[workspace]\nmembers = [\n    \"crates/domain/types\",\n    \"crates/app/core\",\n    \"crates/ports/outbound/traits\",\n    \"crates/adapters/inbound/cli\",\n    \"crates/adapters/outbound/fs\",\n    \"../../packages/shared-types\",\n]\nresolver = \"2\"",
    );
    let results = run_check(tmp.path());
    let errors = arch_01_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("outside") || e.title.contains("shared-types")),
        "expected error about workspace member pointing outside app dir, got: {errors:#?}"
    );
}
