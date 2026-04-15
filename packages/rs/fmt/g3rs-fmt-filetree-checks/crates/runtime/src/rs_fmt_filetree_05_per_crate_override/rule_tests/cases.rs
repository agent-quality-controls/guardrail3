use g3rs_fmt_filetree_checks_assertions::rs_fmt_filetree_05_per_crate_override::rule as assertions;
use test_support::{G3RsFmtConfigFileKind, input};

#[test]
fn errors_for_nested_rustfmt_toml() {
    let mut results = Vec::new();
    super::super::rule::check(&input(
        Some("rustfmt.toml"),
        None,
        vec![(
            "crates/api/rustfmt.toml",
            G3RsFmtConfigFileKind::RustfmtToml,
        )],
        Vec::new(),
    ), &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "Illegal nested rustfmt config",
            "`rustfmt.toml` below repository root is forbidden; rustfmt policy is root-only. Delete this file and ensure all formatting settings are in the root `rustfmt.toml`.",
            "crates/api/rustfmt.toml",
            false,
        )],
    );
}

#[test]
fn errors_for_nested_dot_rustfmt_toml() {
    let mut results = Vec::new();
    super::super::rule::check(&input(
        Some("rustfmt.toml"),
        None,
        vec![(
            "crates/api/.rustfmt.toml",
            G3RsFmtConfigFileKind::DotRustfmtToml,
        )],
        Vec::new(),
    ), &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "Illegal nested rustfmt config",
            "`.rustfmt.toml` below repository root is forbidden; rustfmt policy is root-only. Delete this file and ensure all formatting settings are in the root `rustfmt.toml`.",
            "crates/api/.rustfmt.toml",
            false,
        )],
    );
}
