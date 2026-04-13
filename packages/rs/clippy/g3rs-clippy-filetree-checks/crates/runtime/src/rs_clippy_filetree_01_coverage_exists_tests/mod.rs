use g3rs_clippy_filetree_checks_assertions::rs_clippy_filetree_01_coverage_exists::{
    assert_findings, error, info,
};

use crate::test_support::input;

#[test]
fn inventories_when_workspace_root_has_preferred_clippy_config() {
    let results = crate::check(&input(Some(".clippy.toml"), &[]));

    assert_findings(
        &results,
        &[info(
            "workspace root covered by clippy config",
            "Workspace root is covered by `.clippy.toml`.",
            ".clippy.toml",
            true,
        )],
    );
}

#[test]
fn errors_when_workspace_root_has_no_clippy_config() {
    let results = crate::check(&input(None, &[]));

    assert_findings(
        &results,
        &[error(
            "workspace root uncovered by clippy config",
            "Add `clippy.toml` or `.clippy.toml` at the workspace root so clippy policy is not left to defaults.",
            "clippy.toml",
            false,
        )],
    );
}
