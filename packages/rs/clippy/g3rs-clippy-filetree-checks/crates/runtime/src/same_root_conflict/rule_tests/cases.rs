use g3rs_clippy_filetree_checks_assertions::same_root_conflict::rule::{assert_findings, error};
use test_support::input;

#[test]
fn errors_for_shadowed_plain_clippy_toml_when_dotfile_wins() {
    let mut results = Vec::new();
    super::super::check(
        &input(Some(".clippy.toml"), &[("clippy.toml", ".clippy.toml")]),
        &mut results,
    );

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
    let mut results = Vec::new();
    super::super::check(&input(Some(".clippy.toml"), &[]), &mut results);
    assert_findings(&results, &[]);
}
