use g3rs_fmt_filetree_checks_assertions::dual_file_conflict::rule as assertions;
use test_support::input;

#[test]
fn warns_for_root_dual_file_conflict() {
    let mut results = Vec::new();
    super::super::check(
        &input(
            Some("rustfmt.toml"),
            Some(".rustfmt.toml"),
            Vec::new(),
            vec![""],
        ),
        &mut results,
    );

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "Conflicting rustfmt config files",
            "Both `rustfmt.toml` and `.rustfmt.toml` exist in `.`. Delete `.rustfmt.toml` and keep `rustfmt.toml`.",
            "rustfmt.toml",
            false,
        )],
    );
}

#[test]
fn warns_for_nested_dual_file_conflict() {
    let mut results = Vec::new();
    super::super::check(
        &input(Some("rustfmt.toml"), None, Vec::new(), vec!["crates/api"]),
        &mut results,
    );

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "Conflicting rustfmt config files",
            "Both `rustfmt.toml` and `.rustfmt.toml` exist in `crates/api`. Delete `.rustfmt.toml` and keep `rustfmt.toml`.",
            "crates/api/rustfmt.toml",
            false,
        )],
    );
}
