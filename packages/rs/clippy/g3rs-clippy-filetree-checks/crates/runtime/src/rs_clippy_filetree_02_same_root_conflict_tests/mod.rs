use g3rs_clippy_filetree_checks_assertions::rs_clippy_filetree_02_same_root_conflict::{
    assert_findings, error,
};

use crate::test_support::input;

#[test]
fn errors_for_shadowed_plain_clippy_toml_when_dotfile_wins() {
    let results = crate::check(&input(
        Some(".clippy.toml"),
        &[("clippy.toml", ".clippy.toml")],
    ));

    assert_findings(
        &results,
        &[error(
            "same-root clippy config conflict",
            "`clippy.toml` conflicts with `.clippy.toml` at the same policy root. Keep only the highest-precedence clippy config file.",
            "clippy.toml",
            false,
        )],
    );
}

#[test]
fn stays_quiet_without_shadowed_same_root_configs() {
    let results = crate::check(&input(Some(".clippy.toml"), &[]));
    assert_findings(&results, &[]);
}
