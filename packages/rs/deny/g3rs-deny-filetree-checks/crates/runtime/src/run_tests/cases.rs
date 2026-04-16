use g3rs_deny_filetree_checks_assertions::run as assertions;
use test_support::input;

#[test]
fn run_combines_coverage_and_shadowing_findings() {
    let input = input(
        Some("deny.toml"),
        vec!["deny.toml", ".deny.toml", ".cargo/deny.toml"],
        Vec::new(),
    );

    let results = crate::run::check(&input);

    assertions::assert_combined_coverage_and_shadowing(&results);
}
