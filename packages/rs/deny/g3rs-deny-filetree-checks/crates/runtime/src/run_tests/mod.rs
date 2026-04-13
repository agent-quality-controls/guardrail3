use g3rs_deny_filetree_checks_assertions::{
    rs_deny_filetree_01_coverage as coverage_assertions,
    rs_deny_filetree_03_shadowing as shadowing_assertions,
};

use crate::test_support::input;

#[test]
fn run_combines_coverage_and_shadowing_findings() {
    let input = input(
        Some("deny.toml"),
        vec!["deny.toml", ".deny.toml", ".cargo/deny.toml"],
        Vec::new(),
    );

    let results = crate::check(&input);

    coverage_assertions::assert_findings(
        &results,
        &[coverage_assertions::info(
            "workspace root covered by deny config",
            "workspace root `.` is covered by `deny.toml`.",
            "deny.toml",
            true,
        )],
    );
    shadowing_assertions::assert_findings(
        &results,
        &[shadowing_assertions::error(
            "multiple deny configs at one policy root",
            "`.` has multiple accepted deny configs: .cargo/deny.toml, .deny.toml, deny.toml.",
            ".cargo/deny.toml",
            false,
        )],
    );
}
